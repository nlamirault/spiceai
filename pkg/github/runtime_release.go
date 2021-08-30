package github

import (
	"fmt"
	"runtime"
	"strings"

	"github.com/spiceai/spice/pkg/constants"
)

var (
	assetNameMemo string
	githubClient  = NewGitHubClient(runtimeOwner, runtimeRepo, GetGitHubTokenFromEnv())
)

const (
	runtimeOwner = "spiceai"
	runtimeRepo  = "spiceai"
)

func GetLatestRuntimeRelease(tagName string) (*RepoRelease, error) {
	fmt.Println("Checking for latest Spice runtime release...")

	release, err := GetLatestRelease(githubClient, tagName, GetRuntimeAssetName())
	if err != nil {
		return nil, err
	}

	return release, nil
}

func GetRuntimeVersion(release *RepoRelease) string {
	return strings.TrimSuffix(release.TagName, fmt.Sprintf("-%s", constants.SpiceRuntimeFilename))
}

func DownloadRuntimeAsset(release *RepoRelease, downloadPath string) error {
	assetName := GetRuntimeAssetName()
	return DownloadPrivateReleaseAsset(githubClient, release, assetName, downloadPath)
}

func GetRuntimeAssetName() string {
	if assetNameMemo != "" {
		return assetNameMemo
	}

	assetName := fmt.Sprintf("%s_%s_%s.tar.gz", constants.SpiceRuntimeFilename, runtime.GOOS, runtime.GOARCH)

	assetNameMemo = assetName
	return assetName
}