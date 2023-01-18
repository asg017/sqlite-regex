const fs = require("fs").promises;

module.exports = async ({ github, context }) => {
  const {
    repo: { owner, repo },
    sha,
  } = context;
  console.log(process.env.GITHUB_REF);
  const release = await github.rest.repos.getReleaseByTag({
    owner,
    repo,
    tag: process.env.GITHUB_REF.replace("refs/tags/", ""),
  });

  const release_id = release.data.id;
  async function uploadReleaseAsset(path, name) {
    console.log("Uploading", name, "at", path);

    return github.rest.repos.uploadReleaseAsset({
      owner,
      repo,
      release_id,
      name,
      data: await fs.readFile(path),
    });
  }
  await Promise.all([
    uploadReleaseAsset(
      "sqlite-regex-ubuntu/regex0.so",
      "linux-x86_64-regex0.so"
    ),
    uploadReleaseAsset(
      "sqlite-regex-macos/regex0.dylib",
      "macos-x86_64-regex0.dylib"
    ),
    uploadReleaseAsset(
      "sqlite-regex-macos-arm/regex0.dylib",
      "macos-arm-regex0.dylib"
    ),
    uploadReleaseAsset(
      "sqlite-regex-windows/regex0.dll",
      "windows-x86_64-regex0.dll"
    ),
  ]);

  return;
};
