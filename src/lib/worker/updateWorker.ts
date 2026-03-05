import dayjs from "dayjs";
import { PROD_CONFIG } from "../updates";

export let WORKER_checkforUpdates = async (currentVersion: string) => {
  let updateConfig = PROD_CONFIG;

  console.log("Checking for updates")
  let _res = await fetch(updateConfig.updateGitForge + '/api/v1/repos/' + updateConfig.updateRepo + '/releases')
  let releases: {
    tag_name: string;
    published_at: string;
    html_url: string;
  }[] = await _res.json();

  if (!releases || _res.status !== 200) {
    console.error("Error when checking for updates: " + (releases as any).message);
    postMessage({
      updateAvailable: false
    })
    return;
  }

  let currentRelease = releases.find((r) => r.tag_name == currentVersion);
  let newerReleases = releases.filter((r) => dayjs(r.published_at)
    // if we can't find the current release then just fetch the latest release
    .isAfter((currentRelease) ? currentRelease.published_at : 0)
  );

  if (newerReleases.length < 1) {
    console.log("No updates found")
    postMessage({
      updateAvailable: false
    })

  } else {
    let latestRelease = newerReleases[0];
    console.log(latestRelease.tag_name + " is available!");

    postMessage({
      updateAvailable: true,
      updateUrl: latestRelease.html_url
    })
  }
}

onmessage = (e: any) => {
  WORKER_checkforUpdates(e.data.currentVersion);
}
