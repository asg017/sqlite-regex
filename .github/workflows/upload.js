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
    tag: "unstable",
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
    uploadReleaseAsset("sqlite-regex-ubuntu/libregex0.so", "regex0.so"),
    uploadReleaseAsset("sqlite-regex-macos/libregex0.dylib", "regex0.dylib"),
    uploadReleaseAsset("sqlite-regex-windows/regex0.dll", "regex0.dll"),
  ]);

  return;
};
