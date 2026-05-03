# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.2.8 (2026-05-03)

### Chore

 - <csr-id-00002110cdb9d30d4d251203c2661ef6d82269c6/> add correct binstall metadata
 - <csr-id-00002100a11ca4223fced96e785e8efb670cf722/> move dev-deps closer to deps
 - <csr-id-e9b1fc70adde2a3cbc50920ad56c32a20e00e69f/> update dependency cargo-binstall to v1.19.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.18.1 | 1.19.0 |

### Bug Fixes

 - <csr-id-000020808aaea0268d6508671bc19ece634c72b8/> dedup next servers at the source

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
    - Add correct binstall metadata (0000211)
    - Move dev-deps closer to deps (0000210)
    - Update dependency cargo-binstall to v1.19.0 (e9b1fc7)
    - Dedup next servers at the source (0000208)
</details>

## v1.2.7 (2026-05-02)

<csr-id-0000205024cf6150ba264f744d2360bc70618b9a/>
<csr-id-00002040a5a2b7b25469474dd5a3ea2a37a03e4d/>

### Bug Fixes

 - <csr-id-6593c8cdb547725b831ed5e6770d31a0183ab907/> update hickory-dns monorepo to v0.26.1
   | datasource | package          | from   | to     |
   | ---------- | ---------------- | ------ | ------ |
   | crate      | hickory-net      | 0.26.0 | 0.26.1 |
   | crate      | hickory-proto    | 0.26.0 | 0.26.1 |
   | crate      | hickory-resolver | 0.26.0 | 0.26.1 |

### Other

 - <csr-id-0000205024cf6150ba264f744d2360bc70618b9a/> follow the documentaion on how to create a release
 - <csr-id-00002040a5a2b7b25469474dd5a3ea2a37a03e4d/> this needs the release first

### Chore

 - <csr-id-63bd0051074e5563dce93d083a12b2d4d282d00a/> Bump dnstracer v1.2.7

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump dnstracer v1.2.7 (63bd005)
    - Update hickory-dns monorepo to v0.26.1 (6593c8c)
    - Follow the documentaion on how to create a release (0000205)
    - This needs the release first (0000204)
</details>

## v1.2.6 (2026-04-29)

<csr-id-7c368292770a8ec1c400fff68b3332a286d5dd92/>
<csr-id-000019501570a5d116a3503df36e5f1a3490bd70/>
<csr-id-4eda9890d2e9a92c23abc42a862a47f258634588/>
<csr-id-000020103a72eac413e17d2859610a0781eb3f20/>
<csr-id-00002000c998a043e8e1a39a853a69ab81cf070b/>
<csr-id-000019904b4506ba9baef572c6ad461ed7a79417/>
<csr-id-00001970adb305fb653358a4bcc99cf22f401fcc/>
<csr-id-000019608e5f3db05dcf860f9298c0092990c953/>
<csr-id-b08179348cfdb9e5e2559f9871f6903d45ba1efa/>

### Chore

 - <csr-id-7c368292770a8ec1c400fff68b3332a286d5dd92/> lock file maintenance
 - <csr-id-000019501570a5d116a3503df36e5f1a3490bd70/> set rust-version
 - <csr-id-4eda9890d2e9a92c23abc42a862a47f258634588/> lock file maintenance

### Chore

 - <csr-id-b08179348cfdb9e5e2559f9871f6903d45ba1efa/> Bump dnstracer v1.2.6

### Bug Fixes

 - <csr-id-00002020e2ebbba425111bad2eebfcbf6fc1c801/> add tracing

### Other

 - <csr-id-000020103a72eac413e17d2859610a0781eb3f20/> with release creation
 - <csr-id-00002000c998a043e8e1a39a853a69ab81cf070b/> correct PROJECT_NAME
 - <csr-id-000019904b4506ba9baef572c6ad461ed7a79417/> so much better
 - <csr-id-00001970adb305fb653358a4bcc99cf22f401fcc/> better message
 - <csr-id-000019608e5f3db05dcf860f9298c0092990c953/> add publish via OIDC

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 11 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump dnstracer v1.2.6 (b081793)
    - Add tracing (0000202)
    - With release creation (0000201)
    - Correct PROJECT_NAME (0000200)
    - So much better (0000199)
    - Lock file maintenance (7c36829)
    - Better message (0000197)
    - Add publish via OIDC (0000196)
    - Set rust-version (0000195)
    - Lock file maintenance (4eda989)
</details>

## v1.2.5 (2026-04-17)

<csr-id-b06818faffd51a4565bf4fe8f545d44a1ce5d007/>
<csr-id-5deaed07053c14d52170a1d09952fc11c482a910/>
<csr-id-f01d6af7359e28d9fac62c40d0079173373b87fc/>
<csr-id-b0858c82c672acb4a56555dc9580b9bebf0f2257/>
<csr-id-00001790dfdad3da38f1e54a5b731671de76bdd1/>
<csr-id-8b229ea7be48042620411dc76eaecc2fe3c8190f/>
<csr-id-72bb8b14603d5214d88b6f1cb34b97a01b244686/>
<csr-id-0000180085ec9ea052db908c14b5610909433c86/>
<csr-id-00001780aa6351a86ddf57aaec12fc24014290a0/>

### Chore

 - <csr-id-b06818faffd51a4565bf4fe8f545d44a1ce5d007/> lock file maintenance
 - <csr-id-5deaed07053c14d52170a1d09952fc11c482a910/> lock file maintenance
 - <csr-id-f01d6af7359e28d9fac62c40d0079173373b87fc/> update rust crate insta to v1.47.2
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.47.1 | 1.47.2 |
 - <csr-id-b0858c82c672acb4a56555dc9580b9bebf0f2257/> lock file maintenance
 - <csr-id-00001790dfdad3da38f1e54a5b731671de76bdd1/> typos
 - <csr-id-8b229ea7be48042620411dc76eaecc2fe3c8190f/> update rust crate insta to v1.47.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.47.0 | 1.47.1 |
 - <csr-id-72bb8b14603d5214d88b6f1cb34b97a01b244686/> update rust crate insta to v1.47.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.46.3 | 1.47.0 |

### Bug Fixes

 - <csr-id-00001920619de84cb17b5c340e9d749de6849dd8/> update to hickory-dns 0.26
 - <csr-id-7e23410fde60385da1e092399d0e30192518181a/> update hickory-dns monorepo to v0.26.0
   | datasource | package          | from   | to     |
   | ---------- | ---------------- | ------ | ------ |
   | crate      | hickory-proto    | 0.25.2 | 0.26.0 |
   | crate      | hickory-resolver | 0.25.2 | 0.26.0 |
 - <csr-id-c9e3917634ba0533389f6d4cfd56ca6cdcb1dedd/> update rust crate tokio to v1.52.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.52.0 | 1.52.1 |
 - <csr-id-c287fa0b5c0e0ba6c6ae040154f229c839412267/> update rust crate clap to v4.6.1
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | clap    | 4.6.0 | 4.6.1 |
 - <csr-id-000018803c4b724e1b874c89d8cd48e91ab5ddaf/> delegate formatting to std::fmt::from_fn
 - <csr-id-451739b7f36fa130c57cdc5b52b2a7fd7feec33b/> update rust crate tokio to v1.52.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.51.1 | 1.52.0 |
 - <csr-id-c482bb7c60af4875d2ce0e13b70a2c0d0f46cf4d/> update rust crate tokio to v1.51.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.51.0 | 1.51.1 |
 - <csr-id-ed1eadf96019f319c42c9d1291986d562e979a94/> update rust crate tokio to v1.51.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.50.0 | 1.51.0 |

### Other

 - <csr-id-0000180085ec9ea052db908c14b5610909433c86/> fmt → nightly

### Style

 - <csr-id-00001780aa6351a86ddf57aaec12fc24014290a0/> lints

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 17 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.2.5 (47ca005)
    - Update to hickory-dns 0.26 (0000192)
    - Update hickory-dns monorepo to v0.26.0 (7e23410)
    - Update rust crate tokio to v1.52.1 (c9e3917)
    - Update rust crate clap to v4.6.1 (c287fa0)
    - Delegate formatting to std::fmt::from_fn (0000188)
    - Update rust crate tokio to v1.52.0 (451739b)
    - Lock file maintenance (b06818f)
    - Update rust crate tokio to v1.51.1 (c482bb7)
    - Lock file maintenance (5deaed0)
    - Update rust crate tokio to v1.51.0 (ed1eadf)
    - Update rust crate insta to v1.47.2 (f01d6af)
    - Lock file maintenance (b0858c8)
    - Fmt → nightly (0000180)
    - Typos (0000179)
    - Lints (0000178)
    - Update rust crate insta to v1.47.1 (8b229ea)
    - Update rust crate insta to v1.47.0 (72bb8b1)
</details>

## v1.2.4 (2026-03-25)

<csr-id-000016901c945d4ea5b9d3382023e279e0af29e9/>
<csr-id-5420e34e2468b5a4f0b557819cfe5bfcf01b4b4d/>
<csr-id-dee20ba891fb8c409dbd46e262656e162f92db3f/>
<csr-id-000016400e06131be745c20ff36a380c4890d46a/>
<csr-id-00001740c176a91af33ef9ebef01f9743a8449c8/>
<csr-id-0000166044d71cb9228fb4bbe252a0b88a1da663/>
<csr-id-00001650c0146fcd425e97e565f65cc2c42f71f6/>
<csr-id-00001630e634bfb404ff7bf75471e20b0059988c/>

### Chore

 - <csr-id-000016901c945d4ea5b9d3382023e279e0af29e9/> typo
 - <csr-id-5420e34e2468b5a4f0b557819cfe5bfcf01b4b4d/> lock file maintenance
 - <csr-id-dee20ba891fb8c409dbd46e262656e162f92db3f/> lock file maintenance
 - <csr-id-000016400e06131be745c20ff36a380c4890d46a/> remove unneeded genericity

### Bug Fixes

 - <csr-id-00001730912e5a8e67f2fb9dbf860cdec7010172/> don't buffer too much to show the overview
 - <csr-id-00001720b1a3431a625748d02a09ae9fa8f1ab3a/> don't clobber cnames
 - <csr-id-000017102ca6a7effc581ecff8f0e28cc4029201/> add a recursion limit
 - <csr-id-000017007219a2e6e9310b5c3b65311cfb6b5789/> replace macro is_ip_allowed with a const fn
 - <csr-id-00001620a12891de1b6cf1ba0bbe200f99bc72af/> depth is not really useful here

### Test

 - <csr-id-00001740c176a91af33ef9ebef01f9743a8449c8/> use rstest's Context
 - <csr-id-0000166044d71cb9228fb4bbe252a0b88a1da663/> a bit more
 - <csr-id-00001650c0146fcd425e97e565f65cc2c42f71f6/> test output
 - <csr-id-00001630e634bfb404ff7bf75471e20b0059988c/> a few more tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 10 calendar days.
 - 10 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.2.4 (645b87e)
    - Use rstest's Context (0000174)
    - Don't buffer too much to show the overview (0000173)
    - Don't clobber cnames (0000172)
    - Add a recursion limit (0000171)
    - Replace macro is_ip_allowed with a const fn (0000170)
    - Typo (0000169)
    - Lock file maintenance (5420e34)
    - Lock file maintenance (dee20ba)
    - A bit more (0000166)
    - Test output (0000165)
    - Remove unneeded genericity (0000164)
    - A few more tests (0000163)
    - Depth is not really useful here (0000162)
</details>

## v1.2.3 (2026-03-15)

<csr-id-b6519e912b7f0fee38e365dc5cd11886d83f9b41/>
<csr-id-0000160043e08beb64c622e3d21bb3ad913510d2/>

### Chore

 - <csr-id-b6519e912b7f0fee38e365dc5cd11886d83f9b41/> update rust crate mockall to v0.14.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | mockall | 0.13.1 | 0.14.0 |

### Bug Fixes

 - <csr-id-00001590668e052f8f1a3c5cc8c31140cf096525/> refactor errors and make resolution errors better
 - <csr-id-ba0d47f349da1c30937e3f7b34315c1026686a1c/> update rust crate clap to v4.6.0
   | datasource | package | from   | to    |
   | ---------- | ------- | ------ | ----- |
   | crate      | clap    | 4.5.61 | 4.6.0 |
 - <csr-id-4920b927f77232298a4043058b47c9bdd41e2c2a/> update rust crate clap to v4.5.61
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.60 | 4.5.61 |

### Test

 - <csr-id-0000160043e08beb64c622e3d21bb3ad913510d2/> use expect instead of unwrap

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 3 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.2.3 (d0fdbc6)
    - Use expect instead of unwrap (0000160)
    - Refactor errors and make resolution errors better (0000159)
    - Update rust crate clap to v4.6.0 (ba0d47f)
    - Update rust crate clap to v4.5.61 (4920b92)
    - Update rust crate mockall to v0.14.0 (b6519e9)
</details>

## v1.2.2 (2026-03-11)

<csr-id-432dd98b4b5a161a69c5389dc2554f3ccbda3eb3/>
<csr-id-191d4afef8cbba321a7d9cac0c292572100f1d92/>
<csr-id-4744efdc80728adc20e7334c8390764706e84a4a/>
<csr-id-eabc660f9f004d50103b5c3bca84fa40e2f062f1/>
<csr-id-fa554f7da174bd66aa17d525214468bd219543da/>
<csr-id-00001480ad98c240c5fd8e999d1b2bb7d4bb346d/>
<csr-id-000014702f16cdc21e5a781aa6b48c57405ed52d/>

### Chore

 - <csr-id-432dd98b4b5a161a69c5389dc2554f3ccbda3eb3/> lock file maintenance
 - <csr-id-191d4afef8cbba321a7d9cac0c292572100f1d92/> lock file maintenance
 - <csr-id-4744efdc80728adc20e7334c8390764706e84a4a/> lock file maintenance
 - <csr-id-eabc660f9f004d50103b5c3bca84fa40e2f062f1/> lock file maintenance
 - <csr-id-fa554f7da174bd66aa17d525214468bd219543da/> lock file maintenance

### Bug Fixes

 - <csr-id-00001540a912adf8de03a4c370e7ac4899a82495/> add tests
 - <csr-id-add39482e1a0e6f2bf77caa4b754025ed984d871/> update rust crate tokio to v1.50.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.49.0 | 1.50.0 |
 - <csr-id-000014907e7ed9f8bea1e9e3259636ae9dd92725/> simplifier
 - <csr-id-1fa8492593c53c6d455283e0ca7319215cded8e4/> update rust crate clap to v4.5.60
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.59 | 4.5.60 |
 - <csr-id-80abbaf434a869ca5ca1a7ee60011161fabe9a69/> update rust crate clap to v4.5.59
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.58 | 4.5.59 |
 - <csr-id-b4c47ab88e11adcddee8a9612c5610f99317e60a/> update rust crate clap to v4.5.58
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.57 | 4.5.58 |

### Style

 - <csr-id-00001480ad98c240c5fd8e999d1b2bb7d4bb346d/> lint
 - <csr-id-000014702f16cdc21e5a781aa6b48c57405ed52d/> fmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 28 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.2.2 (fba30bb)
    - Add tests (0000154)
    - Lock file maintenance (432dd98)
    - Update rust crate tokio to v1.50.0 (add3948)
    - Lock file maintenance (191d4af)
    - Lock file maintenance (4744efd)
    - Simplifier (0000149)
    - Lint (0000148)
    - Fmt (0000147)
    - Lock file maintenance (eabc660)
    - Update rust crate clap to v4.5.60 (1fa8492)
    - Update rust crate clap to v4.5.59 (80abbaf)
    - Lock file maintenance (fa554f7)
    - Update rust crate clap to v4.5.58 (b4c47ab)
</details>

## v1.2.1 (2026-02-11)

<csr-id-683a4dcc167754d3eb2d4da2e6b7c8a9f0ac989d/>
<csr-id-7b059778a0f205b02c0ef3021e6fc4eecb3611fb/>
<csr-id-3b67d5ab1c6aa0ba6b5f84a8fd2e90f23f1afdbf/>
<csr-id-7d669f1e1816bd39b18b6f2bdbba4536a543b7d7/>
<csr-id-16260b4307ec1e886e179211580ef63a63d0b5fe/>
<csr-id-26936d4b5da3ee13687440de97c8ddb9b4e6d2b5/>
<csr-id-67c4f1e2f0fa92bab2db9d0a62908787cb358e03/>
<csr-id-0d76b11be1be6a5c109466856f59d5f7745a4295/>
<csr-id-5deb3928a5d2a15c372ed2971232dfc19485b21f/>

### Chore

 - <csr-id-683a4dcc167754d3eb2d4da2e6b7c8a9f0ac989d/> lock file maintenance
 - <csr-id-7b059778a0f205b02c0ef3021e6fc4eecb3611fb/> update rust crate insta to v1.46.3
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.46.2 | 1.46.3 |
 - <csr-id-3b67d5ab1c6aa0ba6b5f84a8fd2e90f23f1afdbf/> lock file maintenance
 - <csr-id-7d669f1e1816bd39b18b6f2bdbba4536a543b7d7/> update rust crate insta to v1.46.2
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.46.1 | 1.46.2 |
 - <csr-id-16260b4307ec1e886e179211580ef63a63d0b5fe/> lock file maintenance
 - <csr-id-26936d4b5da3ee13687440de97c8ddb9b4e6d2b5/> lock file maintenance
 - <csr-id-67c4f1e2f0fa92bab2db9d0a62908787cb358e03/> lock file maintenance
 - <csr-id-0d76b11be1be6a5c109466856f59d5f7745a4295/> update rust crate insta to v1.46.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.46.0 | 1.46.1 |
 - <csr-id-5deb3928a5d2a15c372ed2971232dfc19485b21f/> lock file maintenance

### Bug Fixes

 - <csr-id-b77d73fd406fd5d0325176cf32e014f337c059ec/> update rust crate clap to v4.5.57
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.56 | 4.5.57 |
 - <csr-id-01030373c9f2296858b3793d9477d7d96f510e91/> update rust crate exn to v0.3.0
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | exn     | 0.2.1 | 0.3.0 |
 - <csr-id-370670b2e7e20cab0f69a4e27a8aa10b4b9169c7/> update rust crate clap to v4.5.56
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.55 | 4.5.56 |
 - <csr-id-7bdd596fdb92f9fca5bee1decc19453279bb1e9d/> update rust crate clap to v4.5.55
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.54 | 4.5.55 |
 - <csr-id-28c0fde55f2488993754dc3217eb80b13aa63793/> pin dependencies
   | datasource | package     | from  | to    |
   | ---------- | ----------- | ----- | ----- |
   | crate      | derive_more | 2.1.1 | 2.1.1 |
   | crate      | exn         | 0.2.1 | 0.2.1 |

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release.
 - 34 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.2.1 (62ba7f9)
    - Lock file maintenance (683a4dc)
    - Update rust crate clap to v4.5.57 (b77d73f)
    - Update rust crate insta to v1.46.3 (7b05977)
    - Lock file maintenance (3b67d5a)
    - Update rust crate exn to v0.3.0 (0103037)
    - Update rust crate insta to v1.46.2 (7d669f1)
    - Update rust crate clap to v4.5.56 (370670b)
    - Update rust crate clap to v4.5.55 (7bdd596)
    - Lock file maintenance (16260b4)
    - Lock file maintenance (26936d4)
    - Lock file maintenance (67c4f1e)
    - Update rust crate insta to v1.46.1 (0d76b11)
    - Lock file maintenance (5deb392)
    - Pin dependencies (28c0fde)
</details>

## v1.2.0 (2026-01-07)

<csr-id-8f701e881fbe966034bd089ed7122d73a3852de8/>
<csr-id-12554ca0d8e142b9075f581b9d3c6d2b79888abd/>

### Chore

 - <csr-id-8f701e881fbe966034bd089ed7122d73a3852de8/> lock file maintenance
 - <csr-id-12554ca0d8e142b9075f581b9d3c6d2b79888abd/> update rust crate insta to v1.46.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.45.1 | 1.46.0 |

### New Features

 - <csr-id-000012403ab2dd2955f92e803861c9c1803b7cea/> move from eyre to exn, better errors and smaller runtime

### Bug Fixes

 - <csr-id-00001250ba6458a9c4e3e1b7356a888924303246/> use derive_more a bit more

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.2.0 (52848f8)
    - Use derive_more a bit more (0000125)
    - Move from eyre to exn, better errors and smaller runtime (0000124)
    - Lock file maintenance (8f701e8)
    - Update rust crate insta to v1.46.0 (12554ca)
</details>

## v1.1.10 (2026-01-03)

<csr-id-5538467a65f118bd3e1555f53bb62065e98c37ee/>
<csr-id-acb223281aa88968e18d193b5573e3b674a5d7a8/>
<csr-id-4b68f3b48d7951738852ebf52d410b9c9da65ec2/>
<csr-id-93bb5f21f645c99add608023e0f3e0cfeb0e490a/>
<csr-id-1cf11e0067568701f8680332eed4015f868d10ff/>
<csr-id-9382ad1c6bae1beedc3dc8e0b52b58da3c6bf760/>
<csr-id-e90e7d256941addda900b77bca10a78dfb405c04/>
<csr-id-f630ceb5fba6bcf99ee98596d75ba8584dae808e/>
<csr-id-820e6b8252b1e062f31734a4081bf9c23c989326/>
<csr-id-212e334c50b66cab82dbd3d5abfe48bd3bc2e7ce/>
<csr-id-39987f5032f899fb117210ef234d41f9873bcc54/>

### Chore

 - <csr-id-5538467a65f118bd3e1555f53bb62065e98c37ee/> update rust crate insta to v1.45.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.44.3 | 1.45.1 |
 - <csr-id-acb223281aa88968e18d193b5573e3b674a5d7a8/> lock file maintenance
 - <csr-id-4b68f3b48d7951738852ebf52d410b9c9da65ec2/> lock file maintenance
 - <csr-id-93bb5f21f645c99add608023e0f3e0cfeb0e490a/> lock file maintenance
 - <csr-id-1cf11e0067568701f8680332eed4015f868d10ff/> lock file maintenance
 - <csr-id-9382ad1c6bae1beedc3dc8e0b52b58da3c6bf760/> lock file maintenance
 - <csr-id-e90e7d256941addda900b77bca10a78dfb405c04/> update rust crate insta to v1.44.3
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.44.2 | 1.44.3 |
 - <csr-id-f630ceb5fba6bcf99ee98596d75ba8584dae808e/> update rust crate insta to v1.44.2
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.44.1 | 1.44.2 |
 - <csr-id-820e6b8252b1e062f31734a4081bf9c23c989326/> lock file maintenance
 - <csr-id-212e334c50b66cab82dbd3d5abfe48bd3bc2e7ce/> update rust crate insta to v1.44.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.43.2 | 1.44.1 |
 - <csr-id-39987f5032f899fb117210ef234d41f9873bcc54/> lock file maintenance

### Bug Fixes

 - <csr-id-017d7ec2ce7636e8a52b974fa9e79d3ca4516b7b/> update rust crate tokio to v1.49.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.48.0 | 1.49.0 |
 - <csr-id-cf9e8ff74aca49c7fbf0ce861890a5ed37c0cde1/> update rust crate clap to v4.5.54
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.53 | 4.5.54 |
 - <csr-id-59d864174db2a204a3805d70278785004dd4d928/> update rust crate clap to v4.5.53
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.52 | 4.5.53 |
 - <csr-id-e7d8bfc44a03612e9c6d26267ffd390aed2d9e7c/> update rust crate clap to v4.5.52
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.51 | 4.5.52 |

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 49 days passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.10 (a066ad4)
    - Update rust crate tokio to v1.49.0 (017d7ec)
    - Update rust crate clap to v4.5.54 (cf9e8ff)
    - Update rust crate insta to v1.45.1 (5538467)
    - Lock file maintenance (acb2232)
    - Lock file maintenance (4b68f3b)
    - Lock file maintenance (93bb5f2)
    - Lock file maintenance (1cf11e0)
    - Lock file maintenance (9382ad1)
    - Update rust crate insta to v1.44.3 (e90e7d2)
    - Update rust crate insta to v1.44.2 (f630ceb)
    - Lock file maintenance (820e6b8)
    - Update rust crate insta to v1.44.1 (212e334)
    - Update rust crate clap to v4.5.53 (59d8641)
    - Update rust crate clap to v4.5.52 (e7d8bfc)
    - Lock file maintenance (39987f5)
</details>

## v1.1.9 (2025-11-15)

<csr-id-0000102016da7394fbe44144af54219dc0676f51/>
<csr-id-66f57c3457e4be3ce5d9bac90d3c8a87558b828d/>
<csr-id-083849736e18eb05193a50fdb2d7841cd72a0465/>
<csr-id-0a00c10754bb6ee858715daeb7164c80f33ef348/>
<csr-id-89d5d79c6084439c4f7b0ffc48686eb8fc88634b/>
<csr-id-34add2dec82db0babb8932311445248c904e308c/>
<csr-id-71d60d9aed85a6358e3ebeb810e15c80c172a141/>
<csr-id-7af1fa3a881227b68d2b003bf72a86e3e2ce5f46/>
<csr-id-c8a1409651878f8f0841f0f869d4596f886e73e6/>
<csr-id-175a908e0a00bd17c5a482ba257028d8ad900941/>
<csr-id-8c0c37f78e399a7c42b49e96c295983954ef1934/>
<csr-id-fe4c00656df7cba865b041d89fd82c366fa20b01/>

### Chore

 - <csr-id-0000102016da7394fbe44144af54219dc0676f51/> update lints
 - <csr-id-66f57c3457e4be3ce5d9bac90d3c8a87558b828d/> lock file maintenance
 - <csr-id-083849736e18eb05193a50fdb2d7841cd72a0465/> lock file maintenance
 - <csr-id-0a00c10754bb6ee858715daeb7164c80f33ef348/> lock file maintenance
 - <csr-id-89d5d79c6084439c4f7b0ffc48686eb8fc88634b/> lock file maintenance
 - <csr-id-34add2dec82db0babb8932311445248c904e308c/> lock file maintenance
 - <csr-id-71d60d9aed85a6358e3ebeb810e15c80c172a141/> lock file maintenance
 - <csr-id-7af1fa3a881227b68d2b003bf72a86e3e2ce5f46/> lock file maintenance
 - <csr-id-c8a1409651878f8f0841f0f869d4596f886e73e6/> lock file maintenance
 - <csr-id-175a908e0a00bd17c5a482ba257028d8ad900941/> lock file maintenance
 - <csr-id-8c0c37f78e399a7c42b49e96c295983954ef1934/> lock file maintenance
 - <csr-id-fe4c00656df7cba865b041d89fd82c366fa20b01/> pin rust crate rstest to =0.26.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | rstest  | 0.26.1 | 0.26.1 |

### Bug Fixes

 - <csr-id-00001040632580cd0786210aca14d24875c88bec/> simplify main a bit
 - <csr-id-00001030f0fa75a4bb6566fdba578b8443879390/> get less features from tokio
 - <csr-id-16d0d25801a934667f92398923ce98d95fdf0ced/> update rust crate clap to v4.5.51
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.50 | 4.5.51 |
 - <csr-id-87fa83e2fe9f47df0159fa56aa1b4d651efe3664/> update rust crate clap to v4.5.50
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.49 | 4.5.50 |
 - <csr-id-5e32a8fbb06af5c6324d42162236a0f50ead30f2/> update rust crate tokio to v1.48.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.47.2 | 1.48.0 |
 - <csr-id-4061a62a7e465ffc1cdc83c1250587bb361c9216/> update rust crate tokio to v1.47.2
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.47.1 | 1.47.2 |
 - <csr-id-7e125b9daf14896a73136c88c523c78c3689f054/> update rust crate clap to v4.5.49
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.48 | 4.5.49 |

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release.
 - 19 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.9 (a9ae8ab)
    - Simplify main a bit (0000104)
    - Get less features from tokio (0000103)
    - Update lints (0000102)
    - Lock file maintenance (66f57c3)
    - Lock file maintenance (0838497)
    - Update rust crate clap to v4.5.51 (16d0d25)
    - Lock file maintenance (0a00c10)
    - Update rust crate clap to v4.5.50 (87fa83e)
    - Lock file maintenance (89d5d79)
    - Lock file maintenance (34add2d)
    - Update rust crate tokio to v1.48.0 (5e32a8f)
    - Update rust crate tokio to v1.47.2 (4061a62)
    - Update rust crate clap to v4.5.49 (7e125b9)
    - Lock file maintenance (71d60d9)
    - Lock file maintenance (7af1fa3)
    - Lock file maintenance (c8a1409)
    - Lock file maintenance (175a908)
    - Lock file maintenance (8c0c37f)
    - Pin rust crate rstest to =0.26.1 (fe4c006)
</details>

## v1.1.8 (2025-09-20)

<csr-id-76e66c24a7ba99684b0df875d2d227b14e1e437c/>
<csr-id-c308d13e1284ad30d625b378492cd04f897cab27/>
<csr-id-6e7d52e53b1d83d0cb0e896314fc5676b34b2e11/>
<csr-id-0000084025a902cc26b19c78eab8fd321ff47b9e/>

### Chore

 - <csr-id-76e66c24a7ba99684b0df875d2d227b14e1e437c/> lock file maintenance
 - <csr-id-c308d13e1284ad30d625b378492cd04f897cab27/> lock file maintenance
 - <csr-id-6e7d52e53b1d83d0cb0e896314fc5676b34b2e11/> update rust crate insta to v1.43.2
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.43.1 | 1.43.2 |

### Bug Fixes

 - <csr-id-000008301413329b8017f896c09fab0fe1fe3ce8/> allow passing non uppercase record types
 - <csr-id-edc9246d3578918b98b64b65e004ab0699bc242b/> update rust crate clap to v4.5.48
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.47 | 4.5.48 |

### Test

 - <csr-id-0000084025a902cc26b19c78eab8fd321ff47b9e/> add a few tests, and use rstest+case

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 16 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.8 (daf1f08)
    - Add a few tests, and use rstest+case (0000084)
    - Allow passing non uppercase record types (0000083)
    - Update rust crate clap to v4.5.48 (edc9246)
    - Lock file maintenance (76e66c2)
    - Lock file maintenance (c308d13)
    - Update rust crate insta to v1.43.2 (6e7d52e)
</details>

## v1.1.7 (2025-09-03)

<csr-id-0000077006dc7eee26de87362582c07b0f678a2d/>
<csr-id-c2438fc618d0e3e087bca5fc1d381ddf554bdb0e/>

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

 - 9 commits contributed to the release.
 - 18 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dnstracer v1.1.7 (3c577cd)
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

