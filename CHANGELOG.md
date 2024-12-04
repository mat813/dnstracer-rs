# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.1.2 (2024-12-04)

### Bug Fixes

 - <csr-id-000003473065e9ed637e3f83604a5e530706bd41/> update clap to 4.5.22
 - <csr-id-000003336ec43c3fa57fb74b1547dbbd0c4bc74d/> update tokio to 1.42.0
 - <csr-id-000003275a4427161a70ec1edf463c694c4ec118/> this lifetime has a name

### Other

 - <csr-id-00000354eea0e832fae6285c44605e7222f50652/> include the README.md instead of copying it

### Style

 - <csr-id-00000369560f5a95262d5e2cacc2e38339a7b308/> clippy

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 13 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Clippy (0000036)
    - Include the README.md instead of copying it (0000035)
    - Update clap to 4.5.22 (0000034)
    - Update tokio to 1.42.0 (0000033)
    - This lifetime has a name (0000032)
</details>

## v1.1.1 (2024-11-20)

<csr-id-00000303484ccb4f2fb2180b5f9facdc287fb9e0/>
<csr-id-000002746ba7b4a6356ccc7f5f148913310d3cfb/>

### Chore

 - <csr-id-00000303484ccb4f2fb2180b5f9facdc287fb9e0/> update lock file
 - <csr-id-000002746ba7b4a6356ccc7f5f148913310d3cfb/> clippy

### Bug Fixes

 - <csr-id-0000029a09921484cfb6dca43d40ad0362e23582/> update tokio to 1.41.1
 - <csr-id-000002886d43f0c22549e2d08598c1bc375213db/> update clap to 4.5.21

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 31 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.1 (4f77ad0)
    - Update lock file (0000030)
    - Update tokio to 1.41.1 (0000029)
    - Update clap to 4.5.21 (0000028)
    - Clippy (0000027)
</details>

## v1.1.0 (2024-10-20)

<csr-id-0000020ffd4184a2e3f769b2869d3bc340b60438/>
<csr-id-00000192671e0aeccb2cd9e9b222a38eb495846d/>

### Chore

 - <csr-id-0000020ffd4184a2e3f769b2869d3bc340b60438/> add old changelog

### New Features

 - <csr-id-000002439f930b2237003e14864784265da57ccf/> change the meaning of . as a server
   The default server changed from . to a.root-servers.net.
   If . is used, all root servers will be used.

### Bug Fixes

 - <csr-id-0000025a10a65db7bb515f15b2285d79480e731b/> parse the timeout when the arguments are parsed
 - <csr-id-000002396551c391ae8328059a683779c5af5c27/> use the timeout from the arguments
 - <csr-id-0000022af7f9012d4a22b6b9305eafeea06b57ca/> avoid cloning the last vec for printing
 - <csr-id-0000021d4fdf551a0c1449aba0e788343935d6ce/> simplify code

### Style

 - <csr-id-00000192671e0aeccb2cd9e9b222a38eb495846d/> fmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 1 day passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.0 (76197bf)
    - Parse the timeout when the arguments are parsed (0000025)
    - Change the meaning of . as a server (0000024)
    - Use the timeout from the arguments (0000023)
    - Avoid cloning the last vec for printing (0000022)
    - Simplify code (0000021)
    - Add old changelog (0000020)
    - Fmt (0000019)
</details>

## v1.0.0 (2024-10-19)

<csr-id-000001859882e30eefd707dcb762ccaa67c630cf/>
<csr-id-0000011944a7ba166dff4bdede376f0d1b144af3/>

### Chore

 - <csr-id-000001859882e30eefd707dcb762ccaa67c630cf/> v1.0.0
 - <csr-id-0000011944a7ba166dff4bdede376f0d1b144af3/> lock file maintenance

### New Features

 - <csr-id-00000164ebc05ac97e03ec82c623fa19afc4c630/> add support for TCP queries

### Bug Fixes

 - <csr-id-00000178b20dc72dc70108c5075deb002b97918a/> limit queries when a source address is selected
 - <csr-id-0000015ca8bab3302fbdf5a4b328cc1e9bb8bd94/> keep the original error and don't panic if the client creation fails
 - <csr-id-000001491f9a44fcd6c7cf9f18d2bfb2b0f622b8/> no need to format and parse the SocketAddr, simply create it
 - <csr-id-00000135f1dc569a7a4ffe819f49453d564b0990/> allow binding source address for requests
 - <csr-id-00000127368c25853996083fe7dac003f4015f11/> only format this once

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 1 day passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.0.0 (0000018)
    - Limit queries when a source address is selected (0000017)
    - Add support for TCP queries (0000016)
    - Keep the original error and don't panic if the client creation fails (0000015)
    - No need to format and parse the SocketAddr, simply create it (0000014)
    - Allow binding source address for requests (0000013)
    - Only format this once (0000012)
    - Lock file maintenance (0000011)
</details>

## v0.9.2 (2024-10-17)

<csr-id-0000007442382fb59fc62314a5caae5f024b2f75/>
<csr-id-00000088e4167ccd4c47b2924e7abd4cc470df34/>

### Chore

 - <csr-id-0000007442382fb59fc62314a5caae5f024b2f75/> move a comment at the right place

### Bug Fixes

 - <csr-id-0000010169c90fe6b596fc2f01485441318f4b68/> convert to use all async apis from hickory-dns
 - <csr-id-00000090a38fc33839bcbeda851d01d514cc92fc/> switch to AsyncClient to be able to set edns

### Style

 - <csr-id-00000088e4167ccd4c47b2924e7abd4cc470df34/> clippy

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 day passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Convert to use all async apis from hickory-dns (0000010)
    - Switch to AsyncClient to be able to set edns (0000009)
    - Clippy (0000008)
    - Move a comment at the right place (0000007)
</details>

## v0.9.1 (2024-10-16)

<csr-id-0000005992bae326ff306b9d76952988dd6708ff/>

### Chore

 - <csr-id-0000005992bae326ff306b9d76952988dd6708ff/> add LICENSE file

### Bug Fixes

 - <csr-id-0000006fc0e963dbc8f5176cf0ef150afa840827/> refactor the dns udp lookup in a function

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Refactor the dns udp lookup in a function (0000006)
    - Add LICENSE file (0000005)
</details>

## v0.9.0 (2024-10-13)

<csr-id-0000004e9ef9e2582e57373f8a76beeda2339b36/>
<csr-id-0000003319978b8d3f5ef777cfb41f64b90a21f5/>

### New Features

 - <csr-id-0000002647ef29a540fe23d9cd043773b1cca545/> implement most of the original dnstracer

### Other

 - <csr-id-0000004e9ef9e2582e57373f8a76beeda2339b36/> add ci

### Test

 - <csr-id-0000003319978b8d3f5ef777cfb41f64b90a21f5/> add tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add ci (0000004)
    - Add tests (0000003)
    - Implement most of the original dnstracer (0000002)
    - Init (0000001)
</details>

