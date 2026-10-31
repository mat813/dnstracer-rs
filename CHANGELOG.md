# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.2.12 (2026-10-31)

### Chore

 - <csr-id-81cb72c96cc85336d9a7180efbeece9d78e025a8/> update dependency cargo-nextest to v0.9.150
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.149 | 0.9.150 |
 - <csr-id-b473bbfcb5a3212477f03ebd5ed3f96b1029dbf8/> lock file maintenance
 - <csr-id-c00f757f7142b13b89a619caf6a19c741c3f3753/> update dependency cargo-binstall to v1.26.1
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.26.0 | 1.26.1 |

### Documentation

 - <csr-id-00003360d4d60c75ef9b3e78feb2b8355c282495/> rewrite README in Simplified Technical English
   Also match the options list to the real --help output and use the
   rust-version from Cargo.toml.

### Bug Fixes

 - <csr-id-00003410027b44f7652c26fd18a310c5f3c39d52/> escape control characters in the overview
   The data of a record, for example a TXT string, comes from a name
   server and can contain terminal escape sequences. Escape the control
   characters of each record in the --overview output.
 - <csr-id-00003400a4d14bffc057c92e517cc9af96340ae6/> limit the number of queries of a trace
   A hostile delegation can send an unbounded number of NS and glue
   records, so one trace could send a very large number of queries.
   Add -m/--max-queries (default 200). do_recurse fails with
   MaxQueriesExceeded when the trace reaches the limit.
 - <csr-id-b4f4ca9731d787264b0fd1363d039fb1563f1b64/> update rust crate tokio to v1.54.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.54.0 | 1.54.1 |

### Refactor

 - <csr-id-00003370629991e5c60026713dff6fc48b2207a7/> rewrite comments and error messages in Simplified Technical English

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 10 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Escape control characters in the overview (0000341)
    - Limit the number of queries of a trace (0000340)
    - Update dependency cargo-nextest to v0.9.150 (81cb72c)
    - Lock file maintenance (b473bbf)
    - Rewrite comments and error messages in Simplified Technical English (0000337)
    - Rewrite README in Simplified Technical English (0000336)
    - Update dependency cargo-binstall to v1.26.1 (c00f757)
    - Update rust crate tokio to v1.54.1 (b4f4ca9)
</details>

## v1.2.11 (2026-10-21)

### Chore

 - <csr-id-0000330085418e2be13dc27d25abc16db2b8f290/> drop unused derive_more from_str feature
 - <csr-id-00003270cbb1ad9ca0196ca5a13cd771688f08d6/> drop itertools
   sort_by_cached_key on a Vec replaces the one sorted_by_cached_key call.
 - <csr-id-65fbf7a831ddc46942ca540f3d40577b3c67f3f5/> update dependency cargo-nextest to v0.9.149
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.148 | 0.9.149 |
 - <csr-id-a4e7866cde0f0ac07d90202458e2389fd97b87ed/> update rust crate rstest to v0.28.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | rstest  | 0.27.0 | 0.28.0 |
 - <csr-id-2667b181173150d8b5000688dcb75a1e7796a701/> lock file maintenance
 - <csr-id-6dfb2257180bf03263ab80b7ee26040f237da969/> update dependency cargo-binstall to v1.26.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.25.3 | 1.26.0 |
 - <csr-id-adce51ee49e6fd22d66cbbf0dba2d699803afcca/> update dependency zipsign to v0.2.2
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | zipsign | 0.2.1 | 0.2.2 |
 - <csr-id-b03fdba454f0f2686301380033d6568845201d99/> update dependency rust-lang/rust to v1.99.1
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.99.0 | 1.99.1 |
 - <csr-id-87da601c6cffb663e430d869f14692a0bfd483c9/> update docker.io/library/rust docker tag to v1.99.1
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.99.0 | 1.99.1 |
 - <csr-id-51a6fea2a49b3b92f3e796c461048a7c859fb4eb/> update dependency cargo-sort-derives to v0.14.1
   | datasource | package            | from   | to     |
   | ---------- | ------------------ | ------ | ------ |
   | crate      | cargo-sort-derives | 0.14.0 | 0.14.1 |
 - <csr-id-58e7a626ddf5f6690ff1ba489d8c82dccf2e5168/> lock file maintenance
 - <csr-id-d9496ed9d73016451f9be757a6dafd9dcdc1efc8/> update dependency cargo-sort-derives to v0.14.0
   | datasource | package            | from   | to     |
   | ---------- | ------------------ | ------ | ------ |
   | crate      | cargo-sort-derives | 0.13.0 | 0.14.0 |
 - <csr-id-fd2e521de6654b32e68b4593e439b0c1567e12d1/> update dependency cargo-binstall to v1.25.3
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.25.2 | 1.25.3 |
 - <csr-id-1f6950f949985e8e37c2d42e6bb5d027148e533e/> update dependency cargo-nextest to v0.9.148
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.146 | 0.9.148 |
 - <csr-id-17d5f7f7a4402a54b94b689a30adedf37f2148d8/> update dependency cargo-binstall to v1.25.2
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.25.1 | 1.25.2 |
 - <csr-id-43b25fce7698940978b6058b61fc05907dcfc530/> lock file maintenance
 - <csr-id-34a4f5cca445f052ba115ff092087adb6126c8d0/> update dependency cargo-binstall to v1.25.1
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.25.0 | 1.25.1 |
 - <csr-id-92304494256ed9b2eb3f398aad3242a0f895c5c7/> update rust crate insta to v1.49.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.48.0 | 1.49.0 |
 - <csr-id-a07e4fb348d571cd9d5dd525c747e202cf74b63e/> update dependency cargo-binstall to v1.25.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.24.0 | 1.25.0 |
 - <csr-id-0d83c31b4cbad2d2e268ccbc3a91dbb7c9e3fef7/> update dependency rust-lang/rust to v1.99.0
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.98.1 | 1.99.0 |
 - <csr-id-4bd2511135a482c6ca7f46fd32c6c243244fe26c/> update docker.io/library/rust docker tag to v1.99.0
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.98.1 | 1.99.0 |
 - <csr-id-5c10067f9825bcc88696c6104ee8e1a9019f1f32/> lock file maintenance
 - <csr-id-cb3cee4f35238b1982299840433d25940ba1355b/> update dependency cargo-binstall to v1.24.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.23.0 | 1.24.0 |
 - <csr-id-a6bc2bd29afd8188dbc6e719a8461084df9affc3/> update dependency cargo-nextest to v0.9.146
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.145 | 0.9.146 |
 - <csr-id-82905b90d959faa2f9201f2f507d21f98ca4ac7e/> lock file maintenance
 - <csr-id-fa8ab8afb2d0f327d86b6fe0fccb8489ded20f46/> update dependency cargo-nextest to v0.9.145
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.144 | 0.9.145 |
 - <csr-id-fb24712cca6afab24bc55f210037e2655ee2828f/> lock file maintenance
 - <csr-id-06f04ca92f5bc678f5f98b108d5f28a640d51ff4/> lock file maintenance
 - <csr-id-0455f5c0d31af73a3201517077fd1721545373d9/> update dependency cargo-nextest to v0.9.144
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.143 | 0.9.144 |
 - <csr-id-309ad08bb104c1dbda5e233cd0d379d03be3f07e/> update rust crate rstest to v0.27.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | rstest  | 0.26.1 | 0.27.0 |
 - <csr-id-ce18d1575d893cca82c340c6b81a5fed888676c1/> update docker.io/library/rust docker tag to v1.98.1
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.98.0 | 1.98.1 |
 - <csr-id-560795f004d3a50bcbcc7f5dba16f81d12a184fd/> lock file maintenance
 - <csr-id-2750e4b161f84d2473e977e4ce36dc097a19e7cd/> update dependency cargo-binstall to v1.23.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.22.0 | 1.23.0 |
 - <csr-id-dabaaebf89578606d53ca0dc7c02c2f1d6f8ff33/> update dependency rust-lang/rust to v1.98.1
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.98.0 | 1.98.1 |
 - <csr-id-eeda9f85d5d2824b85c4d5d8a5039d892b7cde1d/> lock file maintenance
 - <csr-id-7c42c865fe8d3f18c6fde36447b4769fc1d1b512/> lock file maintenance
 - <csr-id-33fae1273ce4be224ce32646404f960543c3df0e/> update docker.io/library/rust docker tag to v1.98.0
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.97.1 | 1.98.0 |
 - <csr-id-426e9c79c26e824cabf46bf0ed3fe562f66daaf9/> clippy
 - <csr-id-7ed2d665fea94cd70e81734c5341ada3de13bb77/> update dependency cargo-binstall to v1.22.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.21.1 | 1.22.0 |
 - <csr-id-c6a712579279fe25410e80edacd47f8118ba205f/> update dependency rust-lang/rust to v1.98.0
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.97.1 | 1.98.0 |
 - <csr-id-7d6ad42f35bf221ef8e28127d0219573cdc0da7a/> lock file maintenance
 - <csr-id-7f1db4943d8b22b286f8a2c4dc31c82cd0ccc1da/> lock file maintenance
 - <csr-id-6be4389d2e07fc5432cb71828282d7091c5eab4b/> lock file maintenance
 - <csr-id-22c0bae3a68cebeb88c4ba98a16437b06a7cd913/> update dependency cargo-nextest to v0.9.143
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.140 | 0.9.143 |
 - <csr-id-3e22e521ef7e9569509351454c3853eed16e54f5/> lock file maintenance
 - <csr-id-354de256a1597e84a31ea18d944a78af0a02208b/> lock file maintenance
 - <csr-id-743ef64017df07e75a4ac712c64f81adfdd308bd/> update dependency cargo-binstall to v1.21.1
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.21.0 | 1.21.1 |
 - <csr-id-987409fb9247f11eea5a5168e304b453efc6a411/> lock file maintenance
 - <csr-id-8ff6e5645ab6e7653378491d9c48a4d0bbe32f2f/> lock file maintenance
 - <csr-id-881a90063c6ece64fffe021346de099fe8b8c485/> update docker.io/library/rust docker tag to v1.97.1
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.97.0 | 1.97.1 |
 - <csr-id-b9e3015431de5b83ae81084513a4c49ee1ed5619/> update dependency rust-lang/rust to v1.97.1
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.97.0 | 1.97.1 |
 - <csr-id-3381ad371b73e0e8fefdea91731f79875b2db4c8/> Bump dnstracer v1.2.11

### Bug Fixes

 - <csr-id-a6b6c0d314b2dc8007a5b488029c2e2e61e2ddd1/> update rust crate tokio to v1.54.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.53.3 | 1.54.0 |
 - <csr-id-2d08b71cdb3cbbf2feab17c93fe271667fbc8bd1/> update rust crate tokio to v1.53.3
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.53.2 | 1.53.3 |
 - <csr-id-e8131d343fd51feaac881501ef52d57b90e39d6e/> update rust crate tokio to v1.53.2
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.53.1 | 1.53.2 |
 - <csr-id-8ac1de0975a4752a7dc1f1a84c53b9bc4cf39ffa/> update rust crate clap to v4.6.7
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | clap    | 4.6.6 | 4.6.7 |
 - <csr-id-0070a7cd017d6e1e52d87f9797818595948b79f1/> update hickory-dns monorepo to v0.26.3
   | datasource | package          | from   | to     |
   | ---------- | ---------------- | ------ | ------ |
   | crate      | hickory-net      | 0.26.2 | 0.26.3 |
   | crate      | hickory-proto    | 0.26.2 | 0.26.3 |
   | crate      | hickory-resolver | 0.26.2 | 0.26.3 |
 - <csr-id-aca5ddfd61f02dab5420786e19b5af89bb761292/> update hickory-dns monorepo to v0.26.2
   | datasource | package          | from   | to     |
   | ---------- | ---------------- | ------ | ------ |
   | crate      | hickory-net      | 0.26.1 | 0.26.2 |
   | crate      | hickory-proto    | 0.26.1 | 0.26.2 |
   | crate      | hickory-resolver | 0.26.1 | 0.26.2 |
 - <csr-id-3d2224f651cf47aaa9a92b97063504ad3dea23f5/> update rust crate clap to v4.6.6
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | clap    | 4.6.5 | 4.6.6 |
 - <csr-id-bd14e4bf757d3a798e0b6ae20330ca98cf86d220/> update rust crate clap to v4.6.5
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | clap    | 4.6.4 | 4.6.5 |
 - <csr-id-82fc0e51f3c4b73d2a0af172378584daa71a74fc/> update rust crate clap to v4.6.4
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | clap    | 4.6.3 | 4.6.4 |
 - <csr-id-0adbafb3d1d06b6d8fff7f9090f4dc8e6f74ae02/> update rust crate tokio to v1.53.1
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.53.0 | 1.53.1 |
 - <csr-id-d220b9fd30a8d7086f141fe77a787a9c3fd48299/> update rust crate clap to v4.6.3
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | clap    | 4.6.2 | 4.6.3 |
 - <csr-id-87b50edd0c8639455b8b4486a9343e0e748e6e48/> update rust crate tokio to v1.53.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.52.4 | 1.53.0 |
 - <csr-id-d49878f8f5de0764a65db68481d4805d7d6a4313/> update rust crate tokio to v1.52.4
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.52.3 | 1.52.4 |
 - <csr-id-0518a5d88c758a534c1351ff42dc6de864834fb9/> update rust crate clap to v4.6.2
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | clap    | 4.6.1 | 4.6.2 |

### Other

 - <csr-id-000029803f945b06db2d04949278c2203b249836/> add cargo-sort-derives
 - <csr-id-000029705e248b65d4151f72bd497eeae931c446/> add cargo-sort

### Refactor

 - <csr-id-000033206c0548d705a330b2c6f695c0d0bff340/> build IpAddr from A/AAAA directly
 - <csr-id-00003310f9a24285fdf8d55303e160e0584ec602/> reuse before_len when counting resolved servers
 - <csr-id-000032906ba7b243e397d34171c4f3cd49814a50/> make do_recurse an async fn, box only the recursive call
 - <csr-id-00003280bca96bdf7e73dd3e85c4e8d6c557f1e6/> simplify cache_get
 - <csr-id-00003260d4ffc7b49b25a56559774a79cfec30de/> drop owned From<OptName> for SocketAddr
   Only tests used it.
 - <csr-id-000032505fe43e4a149c7a828ef6b5c73ff08a9c/> derive Default for QueryResult
 - <csr-id-00003240181125679c70cfd89046c97d6b2f5726/> share EDNS and query tail between udp_query and tcp_query
 - <csr-id-00003230a26089d70736b0caf44f0c8ee8dada89/> factor out lookup_servers in resolver
   Three copies of lookup_ip -> filter allowed -> OptName collapse into one
   helper.
 - <csr-id-00003220fe811548f34aef232928ac44683c47cc/> use RefCell instead of RwLock/Mutex in resolver
   Nothing is shared across threads, so drop the lock-poisoning error
   variants and the map_err plumbing. add_result becomes infallible.

### Style

 - <csr-id-0000295019e14ba52387a191eeed7b8d9f6fbfb8/> clippy

### Test

 - <csr-id-0000278053409663888e0ec87598cd844a31e380/> fix snapshots

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 79 commits contributed to the release.
 - 98 days passed between releases.
 - 79 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump dnstracer v1.2.11 (3381ad3)
    - Build IpAddr from A/AAAA directly (0000332)
    - Reuse before_len when counting resolved servers (0000331)
    - Drop unused derive_more from_str feature (0000330)
    - Make do_recurse an async fn, box only the recursive call (0000329)
    - Simplify cache_get (0000328)
    - Drop itertools (0000327)
    - Drop owned From<OptName> for SocketAddr (0000326)
    - Derive Default for QueryResult (0000325)
    - Share EDNS and query tail between udp_query and tcp_query (0000324)
    - Factor out lookup_servers in resolver (0000323)
    - Use RefCell instead of RwLock/Mutex in resolver (0000322)
    - Update rust crate tokio to v1.54.0 (a6b6c0d)
    - Update dependency cargo-nextest to v0.9.149 (65fbf7a)
    - Update rust crate rstest to v0.28.0 (a4e7866)
    - Lock file maintenance (2667b18)
    - Update dependency cargo-binstall to v1.26.0 (6dfb225)
    - Update dependency zipsign to v0.2.2 (adce51e)
    - Update dependency rust-lang/rust to v1.99.1 (b03fdba)
    - Update docker.io/library/rust docker tag to v1.99.1 (87da601)
    - Update rust crate tokio to v1.53.3 (2d08b71)
    - Update dependency cargo-sort-derives to v0.14.1 (51a6fea)
    - Lock file maintenance (58e7a62)
    - Update dependency cargo-sort-derives to v0.14.0 (d9496ed)
    - Update dependency cargo-binstall to v1.25.3 (fd2e521)
    - Update dependency cargo-nextest to v0.9.148 (1f6950f)
    - Update dependency cargo-binstall to v1.25.2 (17d5f7f)
    - Lock file maintenance (43b25fc)
    - Update dependency cargo-binstall to v1.25.1 (34a4f5c)
    - Update rust crate tokio to v1.53.2 (e8131d3)
    - Update rust crate insta to v1.49.0 (9230449)
    - Update dependency cargo-binstall to v1.25.0 (a07e4fb)
    - Update dependency rust-lang/rust to v1.99.0 (0d83c31)
    - Update docker.io/library/rust docker tag to v1.99.0 (4bd2511)
    - Lock file maintenance (5c10067)
    - Add cargo-sort-derives (0000298)
    - Add cargo-sort (0000297)
    - Update dependency cargo-binstall to v1.24.0 (cb3cee4)
    - Clippy (0000295)
    - Update dependency cargo-nextest to v0.9.146 (a6bc2bd)
    - Lock file maintenance (82905b9)
    - Update dependency cargo-nextest to v0.9.145 (fa8ab8a)
    - Update rust crate clap to v4.6.7 (8ac1de0)
    - Lock file maintenance (fb24712)
    - Lock file maintenance (06f04ca)
    - Update dependency cargo-nextest to v0.9.144 (0455f5c)
    - Update hickory-dns monorepo to v0.26.3 (0070a7c)
    - Update rust crate rstest to v0.27.0 (309ad08)
    - Update docker.io/library/rust docker tag to v1.98.1 (ce18d15)
    - Lock file maintenance (560795f)
    - Update dependency cargo-binstall to v1.23.0 (2750e4b)
    - Update hickory-dns monorepo to v0.26.2 (aca5ddf)
    - Update dependency rust-lang/rust to v1.98.1 (dabaaeb)
    - Lock file maintenance (eeda9f8)
    - Lock file maintenance (7c42c86)
    - Fix snapshots (0000278)
    - Update docker.io/library/rust docker tag to v1.98.0 (33fae12)
    - Clippy (426e9c7)
    - Update dependency cargo-binstall to v1.22.0 (7ed2d66)
    - Update dependency rust-lang/rust to v1.98.0 (c6a7125)
    - Lock file maintenance (7d6ad42)
    - Lock file maintenance (7f1db49)
    - Lock file maintenance (6be4389)
    - Update rust crate clap to v4.6.6 (3d2224f)
    - Update dependency cargo-nextest to v0.9.143 (22c0bae)
    - Lock file maintenance (3e22e52)
    - Update rust crate clap to v4.6.5 (bd14e4b)
    - Lock file maintenance (354de25)
    - Update dependency cargo-binstall to v1.21.1 (743ef64)
    - Update rust crate clap to v4.6.4 (82fc0e5)
    - Update rust crate tokio to v1.53.1 (0adbafb)
    - Update rust crate clap to v4.6.3 (d220b9f)
    - Lock file maintenance (987409f)
    - Lock file maintenance (8ff6e56)
    - Update rust crate tokio to v1.53.0 (87b50ed)
    - Update rust crate tokio to v1.52.4 (d49878f)
    - Update docker.io/library/rust docker tag to v1.97.1 (881a900)
    - Update dependency rust-lang/rust to v1.97.1 (b9e3015)
    - Update rust crate clap to v4.6.2 (0518a5d)
</details>

## v1.2.10 (2026-07-15)

### Other

 - <csr-id-0000254077aa9b0c0d77935254c0e608d12ba09d/> add user-agent for crates.io api call

### Chore

 - <csr-id-a5eded9e401f8d6a1167d545c13bb6dcd6236c28/> update dependency cargo-binstall to v1.21.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.20.1 | 1.21.0 |
 - <csr-id-02ebdf1b6fad84d64073be047a0efb1611d8d164/> lock file maintenance
 - <csr-id-4a21532a1fb796d9e931c103701bf48f2caf9ab7/> update docker.io/library/rust docker tag to v1.97.0
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.96.1 | 1.97.0 |
 - <csr-id-58d097cbea7ed9e2e65c7115efd7308aef3d4b44/> update dependency rust-lang/rust to v1.97.0
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.96.1 | 1.97.0 |
 - <csr-id-e600a4f91cf83054d6f9c701d06b3f53959ef9f6/> update dependency rust-lang/rust to v1.96.1
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.96.0 | 1.96.1 |
 - <csr-id-31a7ced9f92d742490478c8f795ee8a62281177f/> update dependency cargo-nextest to v0.9.140
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.138 | 0.9.140 |
 - <csr-id-468edcd70da179a3d3f5f6f6b423a7062a00897d/> lock file maintenance
 - <csr-id-eca9fd11d8ac35a7ba3384fbe0d73acfdf6e58e0/> update docker.io/library/rust docker tag to v1.96.1
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.96.0 | 1.96.1 |
 - <csr-id-545673b549d141a6139dcc4f43486f3c8e57abf6/> update rust crate mockall to v0.15.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | mockall | 0.14.0 | 0.15.0 |
 - <csr-id-bf1caebb5a48f2311b9d9cd03f38e6822726b0f3/> lock file maintenance
 - <csr-id-26e61203bb9d8bee545ad256916a8e490e19c722/> update dependency cargo-nextest to v0.9.138
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.137 | 0.9.138 |
 - <csr-id-d2d73603c0a870b79e47443e022974d76bc70217/> lock file maintenance
 - <csr-id-dae84144caa2b1e3f583f01e8911da5b1d994dc9/> update dependency cargo-binstall to v1.20.1
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.20.0 | 1.20.1 |
 - <csr-id-e8623d0ad27af885bac0a423745274f8a6be90a3/> lock file maintenance
 - <csr-id-7ae2a68f6d51e4c1f9e6f604c02c0297bcfc2b95/> update rust crate insta to v1.48.0
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | insta   | 1.47.2 | 1.48.0 |
 - <csr-id-27d184b67a697d78977c794ccae8a51a1c530253/> update dependency cargo-binstall to v1.20.0
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.19.1 | 1.20.0 |
 - <csr-id-aef066d6aa9791d96b0e5496e51045e14253916d/> lock file maintenance
 - <csr-id-c5e2675ea79949e5de873de8e3b3dbb7b40cad5a/> lock file maintenance
 - <csr-id-de685e073be0be36188eb586ff6b1d73a9aea820/> update dependency rust-lang/rust to v1.96.0
   | datasource      | package        | from   | to     |
   | --------------- | -------------- | ------ | ------ |
   | github-releases | rust-lang/rust | 1.95.0 | 1.96.0 |
 - <csr-id-1fad81e4f1eea7178c48e894afcce160fe0068f5/> update docker.io/library/rust docker tag to v1.96.0
   | datasource | package                | from   | to     |
   | ---------- | ---------------------- | ------ | ------ |
   | docker     | docker.io/library/rust | 1.95.0 | 1.96.0 |
 - <csr-id-70a37d3bc4c086e621367b9b3a91bc4f08d3cf1e/> update dependency cargo-hack to v0.6.45
   | datasource | package    | from   | to     |
   | ---------- | ---------- | ------ | ------ |
   | crate      | cargo-hack | 0.6.44 | 0.6.45 |
 - <csr-id-3c9a45248f90ab820e26d5b9086a1f10255bc9b5/> update dependency cargo-nextest to v0.9.137
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.136 | 0.9.137 |
 - <csr-id-a95bd201071e05bf779fc8fe5a867ecbb68168f1/> lock file maintenance
 - <csr-id-9c07ba1b497caa9673910006fe6985f7809e7989/> lock file maintenance
 - <csr-id-08a916ba6b1bf8a0a51eb66c58c9f38b1bd96c8d/> update dependency cargo-nextest to v0.9.136
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.135 | 0.9.136 |
 - <csr-id-a8558fe1fb7f50dbea5741e2b793403d34bbfa50/> update dependency cargo-nextest to v0.9.135
   | datasource | package       | from    | to      |
   | ---------- | ------------- | ------- | ------- |
   | crate      | cargo-nextest | 0.9.133 | 0.9.135 |
 - <csr-id-bda175141b71adbdf380457880c5ecf0b686b11d/> lock file maintenance
 - <csr-id-0000223022b8ddc577e170f31bb7f750ffc91d38/> fixup release alias
 - <csr-id-435111fcfed04a7ad8a34273be62d36707dd7a29/> update dependency cargo-binstall to v1.19.1
   | datasource | package        | from   | to     |
   | ---------- | -------------- | ------ | ------ |
   | crate      | cargo-binstall | 1.19.0 | 1.19.1 |
 - <csr-id-260bd56bcd8f7ada165292f8fc6ab63233157942/> Bump dnstracer v1.2.10

### Documentation

 - <csr-id-00002190fa3d765bef54cf917c33208ba06f65a1/> some missing doc comments

### Bug Fixes

 - <csr-id-ac6dffdee85cce3956ef937a8e8c033322e1c9da/> update rust crate itertools to v0.15.0
   | datasource | package   | from   | to     |
   | ---------- | --------- | ------ | ------ |
   | crate      | itertools | 0.14.0 | 0.15.0 |
 - <csr-id-189c0255347496bc7a485b07da187f251984f95b/> update rust crate tokio to v1.52.3
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.52.2 | 1.52.3 |
 - <csr-id-7ecc161c5bb12c5b185f3b98f482e232a767324d/> update rust crate exn to v0.3.1
   | datasource | package | from  | to    |
   | ---------- | ------- | ----- | ----- |
   | crate      | exn     | 0.3.0 | 0.3.1 |

### Style

 - <csr-id-0000227041dd4a5c5d4b1389a88b2d2f42828355/> lint

### Test

 - <csr-id-0000218042bb4bf3781d24ea65f127977261766f/> enhance tests a bit

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 37 commits contributed to the release over the course of 69 calendar days.
 - 70 days passed between releases.
 - 37 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add user-agent for crates.io api call (0000254)
    - Bump dnstracer v1.2.10 (260bd56)
    - Update dependency cargo-binstall to v1.21.0 (a5eded9)
    - Lock file maintenance (02ebdf1)
    - Update docker.io/library/rust docker tag to v1.97.0 (4a21532)
    - Update dependency rust-lang/rust to v1.97.0 (58d097c)
    - Update dependency rust-lang/rust to v1.96.1 (e600a4f)
    - Update dependency cargo-nextest to v0.9.140 (31a7ced)
    - Lock file maintenance (468edcd)
    - Update docker.io/library/rust docker tag to v1.96.1 (eca9fd1)
    - Update rust crate mockall to v0.15.0 (545673b)
    - Lock file maintenance (bf1caeb)
    - Update dependency cargo-nextest to v0.9.138 (26e6120)
    - Lock file maintenance (d2d7360)
    - Update dependency cargo-binstall to v1.20.1 (dae8414)
    - Update rust crate itertools to v0.15.0 (ac6dffd)
    - Lock file maintenance (e8623d0)
    - Update rust crate insta to v1.48.0 (7ae2a68)
    - Update dependency cargo-binstall to v1.20.0 (27d184b)
    - Lock file maintenance (aef066d)
    - Lock file maintenance (c5e2675)
    - Update dependency rust-lang/rust to v1.96.0 (de685e0)
    - Update docker.io/library/rust docker tag to v1.96.0 (1fad81e)
    - Update dependency cargo-hack to v0.6.45 (70a37d3)
    - Update dependency cargo-nextest to v0.9.137 (3c9a452)
    - Lock file maintenance (a95bd20)
    - Lock file maintenance (9c07ba1)
    - Lint (0000227)
    - Update dependency cargo-nextest to v0.9.136 (08a916b)
    - Update dependency cargo-nextest to v0.9.135 (a8558fe)
    - Lock file maintenance (bda1751)
    - Fixup release alias (0000223)
    - Update rust crate tokio to v1.52.3 (189c025)
    - Update rust crate exn to v0.3.1 (7ecc161)
    - Update dependency cargo-binstall to v1.19.1 (435111f)
    - Some missing doc comments (0000219)
    - Enhance tests a bit (0000218)
</details>

## v1.2.9 (2026-05-06)

<csr-id-00002160f46d2f1e4d2a6b3ca0d53c904c1bbad2/>
<csr-id-78f5c6adbdc0a5ed347c5c335d9042366d5abe01/>

### Other

 - <csr-id-b463435adb13e8af0609247829d85965ff9111b8/> " Bump dnstracer v1.2.9

### Bug Fixes

 - <csr-id-00002150739c5b6237648ede35f3b09547d926ab/> fix `--server .` usage
   Depending on the resolver used, it may not have an authority section for
   the root zone.
 - <csr-id-26ad2ae79bc55e577532cb9fd302c53c84b3d46c/> update rust crate tokio to v1.52.2
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | tokio   | 1.52.1 | 1.52.2 |

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - " Bump dnstracer v1.2.9 (b463435)
    - Add an alias for release (0000216)
    - Fix `--server .` usage (0000215)
    - Update rust crate tokio to v1.52.2 (26ad2ae)
    - Lock file maintenance (78f5c6a)
</details>

## v1.2.8 (2026-05-03)

<csr-id-00002110cdb9d30d4d251203c2661ef6d82269c6/>
<csr-id-00002100a11ca4223fced96e785e8efb670cf722/>
<csr-id-e9b1fc70adde2a3cbc50920ad56c32a20e00e69f/>
<csr-id-418abaa768e48031d8b676afd2c40aa23d9a2bab/>

### Bug Fixes

 - <csr-id-000020808aaea0268d6508671bc19ece634c72b8/> dedup next servers at the source

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 1 day passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump dnstracer v1.2.8 (418abaa)
    - Add correct binstall metadata (0000211)
    - Move dev-deps closer to deps (0000210)
    - Update dependency cargo-binstall to v1.19.0 (e9b1fc7)
    - Dedup next servers at the source (0000208)
</details>

## v1.2.7 (2026-05-02)

<csr-id-0000205024cf6150ba264f744d2360bc70618b9a/>
<csr-id-00002040a5a2b7b25469474dd5a3ea2a37a03e4d/>
<csr-id-63bd0051074e5563dce93d083a12b2d4d282d00a/>

### Bug Fixes

 - <csr-id-6593c8cdb547725b831ed5e6770d31a0183ab907/> update hickory-dns monorepo to v0.26.1
   | datasource | package          | from   | to     |
   | ---------- | ---------------- | ------ | ------ |
   | crate      | hickory-net      | 0.26.0 | 0.26.1 |
   | crate      | hickory-proto    | 0.26.0 | 0.26.1 |
   | crate      | hickory-resolver | 0.26.0 | 0.26.1 |

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
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

### Bug Fixes

 - <csr-id-00002020e2ebbba425111bad2eebfcbf6fc1c801/> add tracing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 12 days passed between releases.
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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 23 days passed between releases.
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

### Bug Fixes

 - <csr-id-00001730912e5a8e67f2fb9dbf860cdec7010172/> don't buffer too much to show the overview
 - <csr-id-00001720b1a3431a625748d02a09ae9fa8f1ab3a/> don't clobber cnames
 - <csr-id-000017102ca6a7effc581ecff8f0e28cc4029201/> add a recursion limit
 - <csr-id-000017007219a2e6e9310b5c3b65311cfb6b5789/> replace macro is_ip_allowed with a const fn
 - <csr-id-00001620a12891de1b6cf1ba0bbe200f99bc72af/> depth is not really useful here

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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 4 days passed between releases.
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
 - 35 days passed between releases.
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

### New Features

 - <csr-id-000012403ab2dd2955f92e803861c9c1803b7cea/> move from eyre to exn, better errors and smaller runtime

### Bug Fixes

 - <csr-id-00001250ba6458a9c4e3e1b7356a888924303246/> use derive_more a bit more

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 days passed between releases.
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
 - 56 days passed between releases.
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

### Bug Fixes

 - <csr-id-000008301413329b8017f896c09fab0fe1fe3ce8/> allow passing non uppercase record types
 - <csr-id-edc9246d3578918b98b64b65e004ab0699bc242b/> update rust crate clap to v4.5.48
   | datasource | package | from   | to     |
   | ---------- | ------- | ------ | ------ |
   | crate      | clap    | 4.5.47 | 4.5.48 |

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 17 days passed between releases.
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
 - 19 days passed between releases.
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

### Bug Fixes

 - <csr-id-00000670f33fe43499013da8d49cb6f119401cdc/> start using eyre
 - <csr-id-0000066062f322594b011471f65da1d2962f0038/> bump edition to 2024
 - <csr-id-0000064044541e0e1ff95456ca53c4e202a1157a/> update tokio to 1.47.1
 - <csr-id-00000630751e269dfd727fae1e52a690067f141a/> update clap to 4.5.45
 - <csr-id-000006108323bec6cfc8cfa97ca47fb750b6c957/> update hickory* to 0.25.2
 - <csr-id-0000060037d3667a1aae81aca84fa0ab8504dc42/> clippy

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 28 calendar days.
 - 68 days passed between releases.
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

### Bug Fixes

 - <csr-id-000005720c17467d5463df4dc15d077dda828e69/> add a few lints and fix code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 95 days passed between releases.
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
 - 49 days passed between releases.
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

### Bug Fixes

 - <csr-id-0000042136ac438af0dc58a13fe247869bb5cd3a/> update tokio to 1.43.0
 - <csr-id-0000041e456da39e6df21d3a1acc2c4bfa7e5b74/> update itertools to 0.14.0
 - <csr-id-0000040aab4cf270066eb190933f94f877e1ce69/> update hickory-* to 0.24.2
 - <csr-id-0000039d2d445743e950fed09433faa85f537423/> update clap to 4.5.26

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 18 calendar days.
 - 42 days passed between releases.
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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 14 days passed between releases.
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

### New Features

 - <csr-id-000002439f930b2237003e14864784265da57ccf/> change the meaning of . as a server
   The default server changed from . to a.root-servers.net.
   If . is used, all root servers will be used.

### Bug Fixes

 - <csr-id-0000025a10a65db7bb515f15b2285d79480e731b/> parse the timeout when the arguments are parsed
 - <csr-id-000002396551c391ae8328059a683779c5af5c27/> use the timeout from the arguments
 - <csr-id-0000022af7f9012d4a22b6b9305eafeea06b57ca/> avoid cloning the last vec for printing
 - <csr-id-0000021d4fdf551a0c1449aba0e788343935d6ce/> simplify code

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
 - 2 days passed between releases.
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

### Bug Fixes

 - <csr-id-0000010169c90fe6b596fc2f01485441318f4b68/> convert to use all async apis from hickory-dns
 - <csr-id-00000090a38fc33839bcbeda851d01d514cc92fc/> switch to AsyncClient to be able to set edns

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

