/**
 * This script checks for any Alejandra updates and then
 * automatically publishes a new version of the plugin if so.
 */
import { $, CargoToml, semver } from "automation";
import { aiFixAlejandraUpdate } from "./ai_fix.ts";

const rootDirPath = $.path(import.meta.dirname!).parentOrThrow();
const cargoToml = new CargoToml(rootDirPath.join("Cargo.toml"));
const currentVersion = getAlejandraCargoTomlTag(cargoToml.text());

$.logStep("Getting latest version...");
const latestVersion = await getLatestAlejandraVersion();
if (semver.compare(semver.parse(currentVersion), semver.parse(latestVersion)) >= 0) {
  $.log("No new update found. Exiting.");
  Deno.exit(0);
}

$.log(`Found new version: ${currentVersion} -> ${latestVersion}`);
$.logStep("Updating Cargo.toml...");
const isPatchBump = semver.parse(currentVersion).major === semver.parse(latestVersion).major
  && semver.parse(currentVersion).minor === semver.parse(latestVersion).minor;
cargoToml.replaceAll(`tag = "${currentVersion}"`, `tag = "${latestVersion}"`);
// Alejandra is a git dependency, so cargo has to resolve it at the new tag
await $`cargo update -p alejandra`;

// Verify the update. A clean patch bump publishes exactly as before. A minor
// bump always gets an AI review (Alejandra may have added options without
// breaking the build), and a patch bump that fails the checks gets an AI fix attempt.
$.logStep("Running checks (test + clippy + wasm build)...");
const checks = await runChecks();

if (!isPatchBump || !checks.passed) {
  if (checks.passed) {
    $.logStep("Minor Alejandra update — running AI review for new/changed options...");
  } else {
    $.logStep(`${isPatchBump ? "Patch" : "Minor"} update failed the checks — running AI fix...`);
  }
  await aiFixAlejandraUpdate({
    isPatchBump,
    fromVersion: currentVersion,
    toVersion: latestVersion,
    checksPassed: checks.passed,
    // hand the failing output to the AI so it can go straight to fixing
    // instead of re-running the checks just to rediscover the errors.
    checkOutput: checks.output,
  });

  // the AI must leave the project in a passing state, otherwise fail the
  // workflow (nothing gets published and the maintainer is notified).
  $.logStep("Re-running checks after AI changes...");
  await assertChecks();
}

if (Deno.args.includes("--skip-publish")) {
  Deno.exit(0);
}

$.logStep(`Committing Alejandra version bump commit...`);
await $`git add .`;
const message = `${isPatchBump ? "fix" : "feat"}: update to Alejandra ${latestVersion}`;
await $`git commit -m ${message}`;

$.logStep("Bumping version in Cargo.toml...");
// reload from disk before bumping: the AI may have edited Cargo.toml, and the
// in-memory copy loaded at startup is stale. Writing the stale copy here would
// clobber those edits.
const releaseCargoToml = new CargoToml(rootDirPath.join("Cargo.toml"));
releaseCargoToml.bumpCargoTomlVersion(isPatchBump ? "patch" : "minor");

// release
const newVersion = releaseCargoToml.version();
$.logStep(`Committing and publishing ${newVersion}...`);
await $`git add .`;
await $`git commit -m ${newVersion}`;
await $`git push origin main`;
await $`git tag ${newVersion}`;
await $`git push origin ${newVersion}`;

// the checks that must pass before publishing. clippy is included because
// Codex runs it with warnings denied, so a clippy warning is as breaking as a
// test failure, and the wasm release build is included because that is what
// actually ships. `inheritPiped` + `captureCombined` streams the output to the
// CI log live while also capturing it so a failure can be handed to the AI.
async function runChecks(): Promise<{ passed: boolean; output: string }> {
  const results = [];
  for (const command of checkCommands()) {
    results.push(await capture(command()));
  }
  const failures = results.filter((r) => r.code !== 0);
  return {
    passed: failures.length === 0,
    output: failures.map((r) => r.combined).join("\n\n"),
  };
}

function capture(command: ReturnType<typeof $>) {
  return command
    .stdout("inheritPiped")
    .stderr("inheritPiped")
    .captureCombined()
    .noThrow();
}

// same checks as `runChecks`, but throws on the first failure so the workflow
// aborts before anything is committed, tagged, or published.
async function assertChecks(): Promise<void> {
  for (const command of checkCommands()) {
    await command();
  }
}

// a function (not a top-level const) so it is hoisted -- `runChecks` is called
// from top-level code above before a const declared here would be initialized.
function checkCommands() {
  return [
    () => $`cargo test`,
    () => $`cargo clippy --all-targets --all-features -- -D warnings`,
    () => $`cargo build --target wasm32-unknown-unknown --features wasm --release`,
  ];
}

function getAlejandraCargoTomlTag(text: string) {
  const match = text.match(/^alejandra = \{ git = "https:\/\/github\.com\/kamadorueda\/alejandra", tag = "([^"]+)"/m);
  const tag = match?.[1];
  if (tag == null) {
    throw new Error("Could not find the alejandra tag in Cargo.toml.");
  }
  $.logLight("Found alejandra tag in Cargo.toml:", tag);
  return tag;
}

// Alejandra stopped publishing to crates.io after 3.1.0, so its GitHub
// releases are the source of truth
async function getLatestAlejandraVersion() {
  const data = await $.request("https://api.github.com/repos/kamadorueda/alejandra/releases/latest")
    .header("User-Agent", "dprint-plugin-alejandra update script")
    .json<{ tag_name?: string }>();
  const version = data.tag_name;
  if (version == null || semver.tryParse(version) == null) {
    throw new Error(`Could not find the latest release of Alejandra: ${JSON.stringify(data)}`);
  }
  $.logLight("Latest Alejandra release:", version);
  return version;
}
