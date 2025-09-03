# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.1.7 (2025-09-03)

### Chore

 - <csr-id-0000077006dc7eee26de87362582c07b0f678a2d/> remove release profile
 - <csr-id-c2438fc618d0e3e087bca5fc1d381ddf554bdb0e/> lock file maintenance

### Bug Fixes

 - <csr-id-8f5b1fbfa75c34772a0b7840950bfa8d32249380/> update rust crate clap to v4.5.47
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.46 | 4.5.47 |
 - <csr-id-14e5d9898f0bbf09578daf4cb66bfca13b288fa1/> update rust crate clap to v4.5.46
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.45 | 4.5.46 |
 - <csr-id-7072fcf388ab5b40a442c20ba20c1a639257e6e6/> pin dependencies
   | datasource | package          | from   | to     |
   | ---------- | ---------------- | ------ | ------ |
   | crate      | clap             | 4.5.45 | 4.5.45 |
   | crate      | eyre             | 0.6.12 | 0.6.12 |
   | crate      | hickory-client   | 0.25.2 | 0.25.2 |
   | crate      | hickory-proto    | 0.25.2 | 0.25.2 |
   | crate      | hickory-resolver | 0.25.2 | 0.25.2 |
   | crate      | insta            | 1.43.1 | 1.43.1 |
   | crate      | itertools        | 0.14.0 | 0.14.0 |
   | crate      | tokio            | 1.47.1 | 1.47.1 |

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 18 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Remove release profile (0000077)
    - Update rust crate clap to v4.5.47 (8f5b1fb)
    - Lock file maintenance (c2438fc)
    - Update rust crate clap to v4.5.46 (14e5d98)
    - Merge branch 'renovate/pin-dependencies' into 'main' (b80105d)
    - Pin dependencies (7072fcf)
    - Merge branch 'renovate/configure' into 'main' (31df71c)
    - Add renovate.json (51bb575)
</details>

## v1.1.6 (2025-08-15)

<csr-id-00000650455606f53b01ed8e09dea9ff0ab54959/>
<csr-id-00000620f334d9fe7408ea5eab8a175e51352f6d/>
<csr-id-00000680928af94ee73f8139aee2971bddeef129/>

### Chore

 - <csr-id-00000650455606f53b01ed8e09dea9ff0ab54959/> update lock file
 - <csr-id-00000620f334d9fe7408ea5eab8a175e51352f6d/> update lock file

### Bug Fixes

 - <csr-id-00000670f33fe43499013da8d49cb6f119401cdc/> start using eyre
 - <csr-id-0000066062f322594b011471f65da1d2962f0038/> bump edition to 2024
 - <csr-id-0000064044541e0e1ff95456ca53c4e202a1157a/> update tokio to 1.47.1
 - <csr-id-00000630751e269dfd727fae1e52a690067f141a/> update clap to 4.5.45
 - <csr-id-000006108323bec6cfc8cfa97ca47fb750b6c957/> update hickory* to 0.25.2
 - <csr-id-0000060037d3667a1aae81aca84fa0ab8504dc42/> clippy

### Test

 - <csr-id-00000680928af94ee73f8139aee2971bddeef129/> use insta for tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 28 calendar days.
 - 67 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.6 (0265dee)
    - Use insta for tests (0000068)
    - Start using eyre (0000067)
    - Bump edition to 2024 (0000066)
    - Update lock file (0000065)
    - Update tokio to 1.47.1 (0000064)
    - Update clap to 4.5.45 (0000063)
    - Update lock file (0000062)
    - Update hickory* to 0.25.2 (0000061)
    - Clippy (0000060)
</details>

## v1.1.5 (2025-06-08)

<csr-id-000005884cd1d917abd6ebd0ce6319c9d1f4baf9/>
<csr-id-00000563c5592b829463596927684b92e05e91a5/>

### Chore

 - <csr-id-000005884cd1d917abd6ebd0ce6319c9d1f4baf9/> update lock file

### Bug Fixes

 - <csr-id-000005720c17467d5463df4dc15d077dda828e69/> add a few lints and fix code

### Test

 - <csr-id-00000563c5592b829463596927684b92e05e91a5/> fix tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.5 (ec51c27)
    - Update lock file (0000058)
    - Add a few lints and fix code (0000057)
    - Fix tests (0000056)
</details>

## v1.1.4 (2025-03-05)

<csr-id-000005488c8144805e6637de13f294030ae8ff63/>
<csr-id-0000051722ea125a8fb5aaba29ac623a386a5375/>

### Chore

 - <csr-id-000005488c8144805e6637de13f294030ae8ff63/> update lock file
 - <csr-id-0000051722ea125a8fb5aaba29ac623a386a5375/> slim release builds a bit

### Bug Fixes

 - <csr-id-0000053efd45344f286374a3e3cd6a7bb698c657/> update hickory-* to 0.24.4
 - <csr-id-0000052380b4c5c5e3cb56cf55f70b1aa6758c15/> update clap to 4.5.31
 - <csr-id-00000504c6211372a784a144d0cff13a0a69d56b/> refactor the query selection a bit
 - <csr-id-0000049ab80ab44e79da425db9e9328b5539d076/> refactor Display for OptName
 - <csr-id-000004865e295887fcdfae68d26074f59dda7dfe/> use a BTreeSet for results
 - <csr-id-00000471ea9663841e24816a956635d1e38ba17c/> refactor cache with a unified CacheKey
 - <csr-id-0000046592a08e8eb22eff18249901bfccbfc927/> remove some cloning
 - <csr-id-0000045881ef8a7342e01b7acf935c57d0fa58e7/> use is_some_and instead of map_or

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 48 calendar days.
 - 48 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.4 (246f750)
    - Update lock file (0000054)
    - Update hickory-* to 0.24.4 (0000053)
    - Update clap to 4.5.31 (0000052)
    - Slim release builds a bit (0000051)
    - Refactor the query selection a bit (0000050)
    - Refactor Display for OptName (0000049)
    - Use a BTreeSet for results (0000048)
    - Refactor cache with a unified CacheKey (0000047)
    - Remove some cloning (0000046)
    - Use is_some_and instead of map_or (0000045)
</details>

## v1.1.3 (2025-01-15)

<csr-id-000004383b5d7dc2afbb9387ec2e0b32a88145ce/>
<csr-id-0000038bad2d66cd2d526abac3b19289c2f6dfdc/>

### Chore

 - <csr-id-000004383b5d7dc2afbb9387ec2e0b32a88145ce/> update lock file

### Bug Fixes

 - <csr-id-0000042136ac438af0dc58a13fe247869bb5cd3a/> update tokio to 1.43.0
 - <csr-id-0000041e456da39e6df21d3a1acc2c4bfa7e5b74/> update itertools to 0.14.0
 - <csr-id-0000040aab4cf270066eb190933f94f877e1ce69/> update hickory-* to 0.24.2
 - <csr-id-0000039d2d445743e950fed09433faa85f537423/> update clap to 4.5.26

### Other

 - <csr-id-0000038bad2d66cd2d526abac3b19289c2f6dfdc/> build with latest

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 18 calendar days.
 - 41 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.3 (8f57f83)
    - Update lock file (0000043)
    - Update tokio to 1.43.0 (0000042)
    - Update itertools to 0.14.0 (0000041)
    - Update hickory-* to 0.24.2 (0000040)
    - Update clap to 4.5.26 (0000039)
    - Build with latest (0000038)
</details>

## v1.1.2 (2024-12-04)

<csr-id-00000354eea0e832fae6285c44605e7222f50652/>
<csr-id-00000369560f5a95262d5e2cacc2e38339a7b308/>

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

 - 6 commits contributed to the release.
 - 13 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.2 (39b945c)
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

 - 4 commits contributed to the release over the course of 1 calendar day.
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

 - 2 commits contributed to the release over the course of 2 calendar days.
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

