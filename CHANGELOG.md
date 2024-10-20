# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.0.0 (2024-10-19)

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

