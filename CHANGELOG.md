# Changelog

## [0.6.0](https://github.com/leoarayas/mcp-mux/compare/v0.5.0...v0.6.0) (2026-10-03)


### Features

* [@mux](https://github.com/mux) UX + Windows updater fix + minimal-first optimization ([#171](https://github.com/leoarayas/mcp-mux/issues/171)) ([a215012](https://github.com/leoarayas/mcp-mux/commit/a215012ccd37388ffc6d802452e2fe03c9ce1ea5))
* add autostart and system tray functionality ([#38](https://github.com/leoarayas/mcp-mux/issues/38)) ([cc99fcf](https://github.com/leoarayas/mcp-mux/commit/cc99fcf412f24f48edba12b8f0359fa71b5247c6))
* Add custom server configuration fields (env vars, args, headers) ([#54](https://github.com/leoarayas/mcp-mux/issues/54)) ([37ce0f5](https://github.com/leoarayas/mcp-mux/commit/37ce0f575883680e2ee12354e3bfea48e7a9337e))
* add Homebrew tap support and ad-hoc macOS signing ([#79](https://github.com/leoarayas/mcp-mux/issues/79)) ([b07f1a3](https://github.com/leoarayas/mcp-mux/commit/b07f1a3a6a11fd1ae944368fa0909838ffb41292))
* add Linux APT repository and install infrastructure ([#85](https://github.com/leoarayas/mcp-mux/issues/85)) ([473eb1a](https://github.com/leoarayas/mcp-mux/commit/473eb1aeb25c8e01d2111b9e1a82e8fee1a4d4dd))
* add select, file_path, and directory_path input types ([#121](https://github.com/leoarayas/mcp-mux/issues/121)) ([942ee1a](https://github.com/leoarayas/mcp-mux/commit/942ee1ae88f60aa1454bc97cec3839bcacf74454))
* API-key inbound auth for headless/remote MCP clients (P1/3) ([#201](https://github.com/leoarayas/mcp-mux/issues/201)) ([4b5a9bc](https://github.com/leoarayas/mcp-mux/commit/4b5a9bcc73bc301ee7b9591079d7b1166049177a))
* apply McpMux branding to OAuth authorization pages ([#74](https://github.com/leoarayas/mcp-mux/issues/74)) ([c84e036](https://github.com/leoarayas/mcp-mux/commit/c84e036b13b276520b8433439954303dbe3dbaed))
* Capture and stream process stderr to server log manager ([#63](https://github.com/leoarayas/mcp-mux/issues/63)) ([96795b0](https://github.com/leoarayas/mcp-mux/commit/96795b0b54ecfaa9743bb9e6045bfc86ddadcc2f))
* capture MCP protocol logging notifications in server connection logs ([#76](https://github.com/leoarayas/mcp-mux/issues/76)) ([0587741](https://github.com/leoarayas/mcp-mux/commit/058774135fed2c0220a4900372f665e88eb3dff5))
* **featureset:** protect Starter from deletion + clarify mapping popup ([#176](https://github.com/leoarayas/mcp-mux/issues/176)) ([163ee0b](https://github.com/leoarayas/mcp-mux/commit/163ee0b0ef0ac0166be0ecf9e2f8bad1612dfad3))
* file-based keychain fallback for headless Linux/WSL ([#103](https://github.com/leoarayas/mcp-mux/issues/103)) ([9b60e0b](https://github.com/leoarayas/mcp-mux/commit/9b60e0bbe47a2318e7352efd3ba8b1888f393f38))
* **gateway:** default FeatureSet for unmapped roots + Mapped workspaces filter ([#175](https://github.com/leoarayas/mcp-mux/issues/175)) ([7fc50a0](https://github.com/leoarayas/mcp-mux/commit/7fc50a00923f74f752e5c279f60e232c1865c3e5))
* **gateway:** optional network access — bind 0.0.0.0 for LAN sharing ([#200](https://github.com/leoarayas/mcp-mux/issues/200)) ([9e481e7](https://github.com/leoarayas/mcp-mux/commit/9e481e71b5857d9f67b62a811694034bed3a4400))
* generalized client→Space/FeatureSet mappings + lock-confine (P2/3) ([#202](https://github.com/leoarayas/mcp-mux/issues/202)) ([2913ecb](https://github.com/leoarayas/mcp-mux/commit/2913ecb8098044734ea7ee105533d97834f6c956))
* **headless:** add mcpmux-runtime, mcpmuxd daemon and read-only CLI ([c320adb](https://github.com/leoarayas/mcp-mux/commit/c320adb21d43ebb7bf67b3e6f6e0f86e8467309c))
* implement Tauri updater functionality ([#36](https://github.com/leoarayas/mcp-mux/issues/36)) ([d355c68](https://github.com/leoarayas/mcp-mux/commit/d355c68a4b33901adb7f9be8c0765252f8c3577f))
* initial release of McpMux desktop app ([72181e2](https://github.com/leoarayas/mcp-mux/commit/72181e2b462f4f70eb586758e8bd029dcb3b7631))
* Mapping/Clients rename + non-localhost consent note (P3/3) ([#203](https://github.com/leoarayas/mcp-mux/issues/203)) ([87df4a2](https://github.com/leoarayas/mcp-mux/commit/87df4a26fb13e159c09ea8d920c3ce01050c7488))
* per-workspace routing via X-Mcpmux-Workspace header + guided folder setup ([#182](https://github.com/leoarayas/mcp-mux/issues/182)) ([e2ec055](https://github.com/leoarayas/mcp-mux/commit/e2ec0558eada73407addc57902d9f13763cc8aec))
* post-action UX guidance, ConfirmDialog, and client auto-select ([#136](https://github.com/leoarayas/mcp-mux/issues/136)) ([44d934c](https://github.com/leoarayas/mcp-mux/commit/44d934c678c4d7a2eebc996928e2fb37c07d7a8e))
* pre-release update channel + automated pre-releases from main ([#159](https://github.com/leoarayas/mcp-mux/issues/159)) ([e9306c4](https://github.com/leoarayas/mcp-mux/commit/e9306c4a8ac1aee72be2530697a643b69fb130f6))
* redesign README, screenshots, and E2E capture ([#87](https://github.com/leoarayas/mcp-mux/issues/87)) ([84f15cb](https://github.com/leoarayas/mcp-mux/commit/84f15cb8b805912ca810ad00bff9f819802f4d78))
* **spaces:** per-space base directories scope workspace roots to a Space ([#179](https://github.com/leoarayas/mcp-mux/issues/179)) ([fb825cf](https://github.com/leoarayas/mcp-mux/commit/fb825cfe66c6f383ea1f52d290a5519a67bffd5f))
* Streamable HTTP transport with SSE notifications and E2E tests ([#61](https://github.com/leoarayas/mcp-mux/issues/61)) ([ca5b0ff](https://github.com/leoarayas/mcp-mux/commit/ca5b0ffab19aa395a75c5f10a18ab0e6efb1752a))
* support configurable public gateway base URL ([#192](https://github.com/leoarayas/mcp-mux/issues/192)) ([6f81378](https://github.com/leoarayas/mcp-mux/commit/6f81378384ed37cf254d7b8711199404a848ca3f))
* support default values for input definitions ([#70](https://github.com/leoarayas/mcp-mux/issues/70)) ([a1d9599](https://github.com/leoarayas/mcp-mux/commit/a1d9599601c212c1b7054fc4c5c76f065e0ea920))
* **ui:** opencode global connect + client icons ([#184](https://github.com/leoarayas/mcp-mux/issues/184)) ([669e99f](https://github.com/leoarayas/mcp-mux/commit/669e99f2df3d85bbdd33427f0a96bd8e0048c7c3))
* update logo with bolder strokes and regenerate icons/screenshots ([#71](https://github.com/leoarayas/mcp-mux/issues/71)) ([68c292c](https://github.com/leoarayas/mcp-mux/commit/68c292c424671edf4a914484a7af570770fac71e))
* workspace-root routing + Tool Optimization ([@mux](https://github.com/mux)) self-management + UI live-sync ([#151](https://github.com/leoarayas/mcp-mux/issues/151)) ([d614853](https://github.com/leoarayas/mcp-mux/commit/d6148538b6f40644f9367d3c872bc1f4f2f7be63))
* **workspaces:** bulk-clear unmapped folders + clearer approval opt-out ([#172](https://github.com/leoarayas/mcp-mux/issues/172)) ([09b561c](https://github.com/leoarayas/mcp-mux/commit/09b561c901170cac2c6546ef8680b313f292eee6))
* **workspaces:** setting to disable the new-folder mapping prompt ([#177](https://github.com/leoarayas/mcp-mux/issues/177)) ([d5df002](https://github.com/leoarayas/mcp-mux/commit/d5df002df03e9b80a0bd780ce9c52fd9e942d02e))


### Bug Fixes

* add one-click IDE install for VS Code and Cursor ([#119](https://github.com/leoarayas/mcp-mux/issues/119)) ([5b280fb](https://github.com/leoarayas/mcp-mux/commit/5b280fbfdcd04165827b7662ba6896cea96deb83))
* add projectPath to tauri-action for monorepo support ([0299a23](https://github.com/leoarayas/mcp-mux/commit/0299a23c5f995b4bae670ef709134967a19c6ee3))
* add Windsurf, JetBrains, and Android Studio to quick-connect grid ([#139](https://github.com/leoarayas/mcp-mux/issues/139)) ([fb58d9c](https://github.com/leoarayas/mcp-mux/commit/fb58d9ce6c46ec1a55356a9fecb35f34ae2b29f6))
* allow process restart after update and detect Homebrew version mismatch ([#134](https://github.com/leoarayas/mcp-mux/issues/134)) ([ecdbaca](https://github.com/leoarayas/mcp-mux/commit/ecdbacafaff573f497ce6db8614fa39993a28a32))
* avoid synthetic connecting state for enabled servers ([#196](https://github.com/leoarayas/mcp-mux/issues/196)) ([80045c6](https://github.com/leoarayas/mcp-mux/commit/80045c61e9ea2e92a035ab59f6d0adbad33175e0))
* **cargo:** stop Windows-only table from swallowing shared deps ([95ffdad](https://github.com/leoarayas/mcp-mux/commit/95ffdadaa5bc08393283f4e491d60e501ee2d6e6))
* Claude client icon resolving ([#68](https://github.com/leoarayas/mcp-mux/issues/68)) ([c54128e](https://github.com/leoarayas/mcp-mux/commit/c54128e0fbff96bd110de4e4dea45580dfad224c))
* debounce analytics search tracking to capture final query ([#132](https://github.com/leoarayas/mcp-mux/issues/132)) ([0f17ddb](https://github.com/leoarayas/mcp-mux/commit/0f17ddb768b5d309a3a73cc6df492f656e205f69))
* **deps:** resolve 4 transitive security advisories failing Dependabot ([#174](https://github.com/leoarayas/mcp-mux/issues/174)) ([eb32289](https://github.com/leoarayas/mcp-mux/commit/eb32289ca7f309c52223f17cf3a1e1c0f0a61d7c))
* detect OAuth requirement from unexpected content-type responses ([#128](https://github.com/leoarayas/mcp-mux/issues/128)) ([d894d17](https://github.com/leoarayas/mcp-mux/commit/d894d17c7c4c5841b7eb39dc1d7068dbcb447656))
* don't pass APPLE_CERTIFICATE to tauri-action ([1943134](https://github.com/leoarayas/mcp-mux/commit/19431347c63eba3ed00b408d3e6c044bd3ac8a9c))
* e2e flaky fix ([#75](https://github.com/leoarayas/mcp-mux/issues/75)) ([d8e28f8](https://github.com/leoarayas/mcp-mux/commit/d8e28f8fd7a6ab50d3a12d766b970d482ae2fbe2))
* enable createUpdaterArtifacts for updater signatures ([c620e56](https://github.com/leoarayas/mcp-mux/commit/c620e56f9cae7cea4b7682ff49be37da3d1f670e))
* filter invalid DCR redirect URIs ([#193](https://github.com/leoarayas/mcp-mux/issues/193)) ([187b57c](https://github.com/leoarayas/mcp-mux/commit/187b57c31f68519dd536ee8e576cbae0b9a2cf08))
* Fix refresh token issue ([#69](https://github.com/leoarayas/mcp-mux/issues/69)) ([0eba047](https://github.com/leoarayas/mcp-mux/commit/0eba047922d3313121b5dd89e62f8b6aae9fe1db))
* **gateway:** preserve structured tool results ([#206](https://github.com/leoarayas/mcp-mux/issues/206)) ([6bd8220](https://github.com/leoarayas/mcp-mux/commit/6bd822063dc11ff00bf90af18e6d426e37abb84e))
* **gateway:** restore disabled auth on auto-start ([#205](https://github.com/leoarayas/mcp-mux/issues/205)) ([460da1f](https://github.com/leoarayas/mcp-mux/commit/460da1f2b81cb9aad403392d32d3caa2856b8a92))
* **gateway:** ride out self-update port race + clearer update restart UX ([#173](https://github.com/leoarayas/mcp-mux/issues/173)) ([6868992](https://github.com/leoarayas/mcp-mux/commit/6868992faeb77c8fe32ad7c996b2d196ca586002))
* **gateway:** truly no-auth when inbound auth is disabled (no OAuth advertising) ([#187](https://github.com/leoarayas/mcp-mux/issues/187)) ([3e617fd](https://github.com/leoarayas/mcp-mux/commit/3e617fd876360f097fbffa96e21ce1fb90013fea))
* gracefully handle invalid Apple certificate in release builds ([bb4221f](https://github.com/leoarayas/mcp-mux/commit/bb4221f9e4a47ff7fad041b13e432a2ed55e1f96))
* make feature discovery capability-aware and bounded ([#194](https://github.com/leoarayas/mcp-mux/issues/194)) ([c2a1569](https://github.com/leoarayas/mcp-mux/commit/c2a156974004e113b4368dada41411e61c8205f1))
* **oauth:** DCR skip-invalid redirect URIs + drop duplicate RFC 8707 resource param ([#158](https://github.com/leoarayas/mcp-mux/issues/158)) ([661f162](https://github.com/leoarayas/mcp-mux/commit/661f1620105803acfe07087e997a1d4d00aa77d5))
* **oauth:** de-duplicate deep-link handling + quiet status-poll log ([#189](https://github.com/leoarayas/mcp-mux/issues/189)) ([9dd7b58](https://github.com/leoarayas/mcp-mux/commit/9dd7b58a23cef1f7f6a5ce53cab20750270c1a26))
* **oauth:** remove credential caching to enable automatic token refresh ([#33](https://github.com/leoarayas/mcp-mux/issues/33)) ([f398cfa](https://github.com/leoarayas/mcp-mux/commit/f398cfad7f1f92956f528b5e4640049de77b5ac3))
* regenerate ICO with proper sizes & increase connection timeout ([#123](https://github.com/leoarayas/mcp-mux/issues/123)) ([2d88b25](https://github.com/leoarayas/mcp-mux/commit/2d88b259e9ca1bbc1ac57405854d732d8437cce3))
* remove notarization env vars from tauri-action ([31ed4ed](https://github.com/leoarayas/mcp-mux/commit/31ed4ed7a1aa8b14bb6f403939fa62652a1579d3))
* render server icon URLs as images instead of raw text ([#57](https://github.com/leoarayas/mcp-mux/issues/57)) ([5a94708](https://github.com/leoarayas/mcp-mux/commit/5a94708dcddd47c26183bd18c6abd348c91f976c))
* replace deprecated macos-13 runner with macos-latest ([92ad770](https://github.com/leoarayas/mcp-mux/commit/92ad7702d2cada69df46c3a221bc2101caf17a20))
* resolve npx/node PATH on macOS GUI apps ([#113](https://github.com/leoarayas/mcp-mux/issues/113)) ([98c013d](https://github.com/leoarayas/mcp-mux/commit/98c013d4e6955e678949df6068c038e1b8cf00fc))
* restore titlebar drag region without breaking controls ([#197](https://github.com/leoarayas/mcp-mux/issues/197)) ([47c787c](https://github.com/leoarayas/mcp-mux/commit/47c787cf95b1c36e99673596170666252c59f114))
* **servers:** pin config-modal footer ([#163](https://github.com/leoarayas/mcp-mux/issues/163)) + silent Windows updates ([#165](https://github.com/leoarayas/mcp-mux/issues/165)) ([0ddbdb5](https://github.com/leoarayas/mcp-mux/commit/0ddbdb59c7229f8bd9dd0a4875216af5ab8977af))
* skip Apple certificate in tauri-action when import fails ([968d4b9](https://github.com/leoarayas/mcp-mux/commit/968d4b90b09bffc9a2432a32f2318ffde3facb88))
* **spaces:** clearer base-directories UX ([#180](https://github.com/leoarayas/mcp-mux/issues/180)) ([4a69908](https://github.com/leoarayas/mcp-mux/commit/4a699085087b775439180ace2467cda708b26fc3))
* stdio enable error UI state ([#104](https://github.com/leoarayas/mcp-mux/issues/104)) ([b4598e6](https://github.com/leoarayas/mcp-mux/commit/b4598e60e12d3389717fc2252bac8eb29e96f9c9))
* **storage:** drop a deleted FeatureSet from workspace bindings ([#186](https://github.com/leoarayas/mcp-mux/issues/186)) ([5598451](https://github.com/leoarayas/mcp-mux/commit/559845193aae3bc5bffff1e41758ffce1c083625))
* **storage:** purge orphaned feature_set_members after the refactor (migration 017) ([#167](https://github.com/leoarayas/mcp-mux/issues/167)) ([b90b05c](https://github.com/leoarayas/mcp-mux/commit/b90b05c038d9d1bc8fae395e83e0e8db713d3e3f))
* suppress console window for stdio MCP servers on Windows ([#59](https://github.com/leoarayas/mcp-mux/issues/59)) ([98f862c](https://github.com/leoarayas/mcp-mux/commit/98f862cae83f24c4397fe8e6204215c68b0baf92))
* sync custom server config saves immediately ([#195](https://github.com/leoarayas/mcp-mux/issues/195)) ([536a4ce](https://github.com/leoarayas/mcp-mux/commit/536a4ceb7e4ad32549f870145557d00fe790cb2e))
* taskbar icon visibility ([#83](https://github.com/leoarayas/mcp-mux/issues/83)) ([400c4bc](https://github.com/leoarayas/mcp-mux/commit/400c4bcce315bc7354dacb40b1cfa95a51e0edd3))
* **ui:** scroll-to + flash the target Settings section on every redirect ([#190](https://github.com/leoarayas/mcp-mux/issues/190)) ([e032c9b](https://github.com/leoarayas/mcp-mux/commit/e032c9bc2e5a99ea64df4a4af215c5a29153af52))
* **ui:** show official opencode logo in the Apps tab ([#185](https://github.com/leoarayas/mcp-mux/issues/185)) ([608a841](https://github.com/leoarayas/mcp-mux/commit/608a841b1b0d058e055ff2ff0ba6cc37ed921a33))
* Update screenshots and Logo ([#56](https://github.com/leoarayas/mcp-mux/issues/56)) ([e6fb736](https://github.com/leoarayas/mcp-mux/commit/e6fb736ca7a79c13f227cdb470e735df447ea7cd))
* ux improvements and fixes ([#42](https://github.com/leoarayas/mcp-mux/issues/42)) ([fa52576](https://github.com/leoarayas/mcp-mux/commit/fa52576fc79102af992f71ef059f8f7eb937a23d))
* version display & update check ([#117](https://github.com/leoarayas/mcp-mux/issues/117)) ([b40c59b](https://github.com/leoarayas/mcp-mux/commit/b40c59bfb7b9ec19be8848abe04e38ba6fed1422))
* **windows:** address review findings on the stdio shutdown path ([96045be](https://github.com/leoarayas/mcp-mux/commit/96045be5c661332f03c1fc39e574c84bd84da547))
* **windows:** keep MCP stdio children console-free and awaited ([0e21d6f](https://github.com/leoarayas/mcp-mux/commit/0e21d6f78f579f6920435ba7aa235c47866f0b12))
* **windows:** terminate gateway child process trees on exit ([9d1b511](https://github.com/leoarayas/mcp-mux/commit/9d1b51160078307394a05d665d9ed6e4cbcbd1d8))
* wire up HTTP definition headers orthogonally from auth ([#125](https://github.com/leoarayas/mcp-mux/issues/125)) ([04380e0](https://github.com/leoarayas/mcp-mux/commit/04380e0979ab428351185d381001d209e6a4993b))


### Refactoring

* remove Password and Textarea from InputType enum ([#122](https://github.com/leoarayas/mcp-mux/issues/122)) ([bd06386](https://github.com/leoarayas/mcp-mux/commit/bd06386e04020da381135761a631ab38543ae414))


### Documentation

* add comprehensive light-theme screenshots for all features ([#47](https://github.com/leoarayas/mcp-mux/issues/47)) ([cefa644](https://github.com/leoarayas/mcp-mux/commit/cefa644daad6c23640db6bc767eb1dd0e43199f0))
* add Discord community link to README ([#149](https://github.com/leoarayas/mcp-mux/issues/149)) ([c32f78f](https://github.com/leoarayas/mcp-mux/commit/c32f78f7143177589ea96b4e33170f49cc343b30))
* add mcpmux.com links and download references to README ([#64](https://github.com/leoarayas/mcp-mux/issues/64)) ([04ab100](https://github.com/leoarayas/mcp-mux/commit/04ab1006d68860ff6f347ef6cd68dd3d678c3352))
* add user guide with screenshots ([#130](https://github.com/leoarayas/mcp-mux/issues/130)) ([a97a133](https://github.com/leoarayas/mcp-mux/commit/a97a1333520fc1ac54f061344970cf493807ca87))
* add user guide with screenshots ([#131](https://github.com/leoarayas/mcp-mux/issues/131)) ([ee28e8b](https://github.com/leoarayas/mcp-mux/commit/ee28e8be432d2b1532f3f98067ba9004c4a18374))
* add Workspaces and Tool Optimization guides ([#164](https://github.com/leoarayas/mcp-mux/issues/164)) ([25a6fbb](https://github.com/leoarayas/mcp-mux/commit/25a6fbb94d3efcbd0cd5a714ac452c5e61608250))
* complete the getting-started flow + workspace-driven routing ([#166](https://github.com/leoarayas/mcp-mux/issues/166)) ([92f8ac2](https://github.com/leoarayas/mcp-mux/commit/92f8ac2f053e27b4a1aec222eea7ff3f9986559c))
* comprehensive README rewrite with features, security, and archi… ([#44](https://github.com/leoarayas/mcp-mux/issues/44)) ([243d3a3](https://github.com/leoarayas/mcp-mux/commit/243d3a369f34cfb02c5e82085e90c68e5bed963d))
* improve README first impression with problem/fix diagrams ([#109](https://github.com/leoarayas/mcp-mux/issues/109)) ([b15482b](https://github.com/leoarayas/mcp-mux/commit/b15482b32a016e3ca92753f26212f5827f744903))
* record the Windows stdio process containment investigation ([dc2318b](https://github.com/leoarayas/mcp-mux/commit/dc2318b63a570e77371a7d34e372702aea7ff1f4))

## [0.5.0](https://github.com/mcpmux/mcp-mux/compare/v0.4.0...v0.5.0) (2026-06-25)


### Features

* per-workspace routing via X-Mcpmux-Workspace header + guided folder setup ([#182](https://github.com/mcpmux/mcp-mux/issues/182)) ([e2ec055](https://github.com/mcpmux/mcp-mux/commit/e2ec0558eada73407addc57902d9f13763cc8aec))
* **ui:** opencode global connect + client icons ([#184](https://github.com/mcpmux/mcp-mux/issues/184)) ([669e99f](https://github.com/mcpmux/mcp-mux/commit/669e99f2df3d85bbdd33427f0a96bd8e0048c7c3))


### Bug Fixes

* **gateway:** truly no-auth when inbound auth is disabled (no OAuth advertising) ([#187](https://github.com/mcpmux/mcp-mux/issues/187)) ([3e617fd](https://github.com/mcpmux/mcp-mux/commit/3e617fd876360f097fbffa96e21ce1fb90013fea))
* **oauth:** de-duplicate deep-link handling + quiet status-poll log ([#189](https://github.com/mcpmux/mcp-mux/issues/189)) ([9dd7b58](https://github.com/mcpmux/mcp-mux/commit/9dd7b58a23cef1f7f6a5ce53cab20750270c1a26))
* **storage:** drop a deleted FeatureSet from workspace bindings ([#186](https://github.com/mcpmux/mcp-mux/issues/186)) ([5598451](https://github.com/mcpmux/mcp-mux/commit/559845193aae3bc5bffff1e41758ffce1c083625))
* **ui:** scroll-to + flash the target Settings section on every redirect ([#190](https://github.com/mcpmux/mcp-mux/issues/190)) ([e032c9b](https://github.com/mcpmux/mcp-mux/commit/e032c9bc2e5a99ea64df4a4af215c5a29153af52))
* **ui:** show official opencode logo in the Apps tab ([#185](https://github.com/mcpmux/mcp-mux/issues/185)) ([608a841](https://github.com/mcpmux/mcp-mux/commit/608a841b1b0d058e055ff2ff0ba6cc37ed921a33))

## [0.4.0](https://github.com/mcpmux/mcp-mux/compare/v0.3.0...v0.4.0) (2026-06-19)


### Features

* [@mux](https://github.com/mux) UX + Windows updater fix + minimal-first optimization ([#171](https://github.com/mcpmux/mcp-mux/issues/171)) ([a215012](https://github.com/mcpmux/mcp-mux/commit/a215012ccd37388ffc6d802452e2fe03c9ce1ea5))
* **featureset:** protect Starter from deletion + clarify mapping popup ([#176](https://github.com/mcpmux/mcp-mux/issues/176)) ([163ee0b](https://github.com/mcpmux/mcp-mux/commit/163ee0b0ef0ac0166be0ecf9e2f8bad1612dfad3))
* **gateway:** default FeatureSet for unmapped roots + Mapped workspaces filter ([#175](https://github.com/mcpmux/mcp-mux/issues/175)) ([7fc50a0](https://github.com/mcpmux/mcp-mux/commit/7fc50a00923f74f752e5c279f60e232c1865c3e5))
* pre-release update channel + automated pre-releases from main ([#159](https://github.com/mcpmux/mcp-mux/issues/159)) ([e9306c4](https://github.com/mcpmux/mcp-mux/commit/e9306c4a8ac1aee72be2530697a643b69fb130f6))
* **spaces:** per-space base directories scope workspace roots to a Space ([#179](https://github.com/mcpmux/mcp-mux/issues/179)) ([fb825cf](https://github.com/mcpmux/mcp-mux/commit/fb825cfe66c6f383ea1f52d290a5519a67bffd5f))
* workspace-root routing + Tool Optimization ([@mux](https://github.com/mux)) self-management + UI live-sync ([#151](https://github.com/mcpmux/mcp-mux/issues/151)) ([d614853](https://github.com/mcpmux/mcp-mux/commit/d6148538b6f40644f9367d3c872bc1f4f2f7be63))
* **workspaces:** bulk-clear unmapped folders + clearer approval opt-out ([#172](https://github.com/mcpmux/mcp-mux/issues/172)) ([09b561c](https://github.com/mcpmux/mcp-mux/commit/09b561c901170cac2c6546ef8680b313f292eee6))
* **workspaces:** setting to disable the new-folder mapping prompt ([#177](https://github.com/mcpmux/mcp-mux/issues/177)) ([d5df002](https://github.com/mcpmux/mcp-mux/commit/d5df002df03e9b80a0bd780ce9c52fd9e942d02e))


### Bug Fixes

* add Windsurf, JetBrains, and Android Studio to quick-connect grid ([#139](https://github.com/mcpmux/mcp-mux/issues/139)) ([fb58d9c](https://github.com/mcpmux/mcp-mux/commit/fb58d9ce6c46ec1a55356a9fecb35f34ae2b29f6))
* **deps:** resolve 4 transitive security advisories failing Dependabot ([#174](https://github.com/mcpmux/mcp-mux/issues/174)) ([eb32289](https://github.com/mcpmux/mcp-mux/commit/eb32289ca7f309c52223f17cf3a1e1c0f0a61d7c))
* **gateway:** ride out self-update port race + clearer update restart UX ([#173](https://github.com/mcpmux/mcp-mux/issues/173)) ([6868992](https://github.com/mcpmux/mcp-mux/commit/6868992faeb77c8fe32ad7c996b2d196ca586002))
* **oauth:** DCR skip-invalid redirect URIs + drop duplicate RFC 8707 resource param ([#158](https://github.com/mcpmux/mcp-mux/issues/158)) ([661f162](https://github.com/mcpmux/mcp-mux/commit/661f1620105803acfe07087e997a1d4d00aa77d5))
* **servers:** pin config-modal footer ([#163](https://github.com/mcpmux/mcp-mux/issues/163)) + silent Windows updates ([#165](https://github.com/mcpmux/mcp-mux/issues/165)) ([0ddbdb5](https://github.com/mcpmux/mcp-mux/commit/0ddbdb59c7229f8bd9dd0a4875216af5ab8977af))
* **spaces:** clearer base-directories UX ([#180](https://github.com/mcpmux/mcp-mux/issues/180)) ([4a69908](https://github.com/mcpmux/mcp-mux/commit/4a699085087b775439180ace2467cda708b26fc3))
* **storage:** purge orphaned feature_set_members after the refactor (migration 017) ([#167](https://github.com/mcpmux/mcp-mux/issues/167)) ([b90b05c](https://github.com/mcpmux/mcp-mux/commit/b90b05c038d9d1bc8fae395e83e0e8db713d3e3f))


### Documentation

* add Discord community link to README ([#149](https://github.com/mcpmux/mcp-mux/issues/149)) ([c32f78f](https://github.com/mcpmux/mcp-mux/commit/c32f78f7143177589ea96b4e33170f49cc343b30))
* add Workspaces and Tool Optimization guides ([#164](https://github.com/mcpmux/mcp-mux/issues/164)) ([25a6fbb](https://github.com/mcpmux/mcp-mux/commit/25a6fbb94d3efcbd0cd5a714ac452c5e61608250))
* complete the getting-started flow + workspace-driven routing ([#166](https://github.com/mcpmux/mcp-mux/issues/166)) ([92f8ac2](https://github.com/mcpmux/mcp-mux/commit/92f8ac2f053e27b4a1aec222eea7ff3f9986559c))

## [0.3.0](https://github.com/mcpmux/mcp-mux/compare/v0.2.3...v0.3.0) (2026-02-25)


### Features

* post-action UX guidance, ConfirmDialog, and client auto-select ([#136](https://github.com/mcpmux/mcp-mux/issues/136)) ([44d934c](https://github.com/mcpmux/mcp-mux/commit/44d934c678c4d7a2eebc996928e2fb37c07d7a8e))

## [0.2.3](https://github.com/mcpmux/mcp-mux/compare/v0.2.2...v0.2.3) (2026-02-21)


### Bug Fixes

* allow process restart after update and detect Homebrew version mismatch ([#134](https://github.com/mcpmux/mcp-mux/issues/134)) ([ecdbaca](https://github.com/mcpmux/mcp-mux/commit/ecdbacafaff573f497ce6db8614fa39993a28a32))
* debounce analytics search tracking to capture final query ([#132](https://github.com/mcpmux/mcp-mux/issues/132)) ([0f17ddb](https://github.com/mcpmux/mcp-mux/commit/0f17ddb768b5d309a3a73cc6df492f656e205f69))

## [0.2.2](https://github.com/mcpmux/mcp-mux/compare/v0.2.1...v0.2.2) (2026-02-20)


### Bug Fixes

* detect OAuth requirement from unexpected content-type responses ([#128](https://github.com/mcpmux/mcp-mux/issues/128)) ([d894d17](https://github.com/mcpmux/mcp-mux/commit/d894d17c7c4c5841b7eb39dc1d7068dbcb447656))
* wire up HTTP definition headers orthogonally from auth ([#125](https://github.com/mcpmux/mcp-mux/issues/125)) ([04380e0](https://github.com/mcpmux/mcp-mux/commit/04380e0979ab428351185d381001d209e6a4993b))


### Documentation

* add user guide with screenshots ([#130](https://github.com/mcpmux/mcp-mux/issues/130)) ([a97a133](https://github.com/mcpmux/mcp-mux/commit/a97a1333520fc1ac54f061344970cf493807ca87))
* add user guide with screenshots ([#131](https://github.com/mcpmux/mcp-mux/issues/131)) ([ee28e8b](https://github.com/mcpmux/mcp-mux/commit/ee28e8be432d2b1532f3f98067ba9004c4a18374))

## [0.2.1](https://github.com/mcpmux/mcp-mux/compare/v0.2.0...v0.2.1) (2026-02-19)


### Bug Fixes

* regenerate ICO with proper sizes & increase connection timeout ([#123](https://github.com/mcpmux/mcp-mux/issues/123)) ([2d88b25](https://github.com/mcpmux/mcp-mux/commit/2d88b259e9ca1bbc1ac57405854d732d8437cce3))


### Refactoring

* remove Password and Textarea from InputType enum ([#122](https://github.com/mcpmux/mcp-mux/issues/122)) ([bd06386](https://github.com/mcpmux/mcp-mux/commit/bd06386e04020da381135761a631ab38543ae414))

## [0.2.0](https://github.com/mcpmux/mcp-mux/compare/v0.1.2...v0.2.0) (2026-02-18)


### Features

* add select, file_path, and directory_path input types ([#121](https://github.com/mcpmux/mcp-mux/issues/121)) ([942ee1a](https://github.com/mcpmux/mcp-mux/commit/942ee1ae88f60aa1454bc97cec3839bcacf74454))


### Bug Fixes

* add one-click IDE install for VS Code and Cursor ([#119](https://github.com/mcpmux/mcp-mux/issues/119)) ([5b280fb](https://github.com/mcpmux/mcp-mux/commit/5b280fbfdcd04165827b7662ba6896cea96deb83))
* version display & update check ([#117](https://github.com/mcpmux/mcp-mux/issues/117)) ([b40c59b](https://github.com/mcpmux/mcp-mux/commit/b40c59bfb7b9ec19be8848abe04e38ba6fed1422))

## [0.1.2](https://github.com/mcpmux/mcp-mux/compare/v0.1.1...v0.1.2) (2026-02-18)


### Bug Fixes

* resolve npx/node PATH on macOS GUI apps ([#113](https://github.com/mcpmux/mcp-mux/issues/113)) ([98c013d](https://github.com/mcpmux/mcp-mux/commit/98c013d4e6955e678949df6068c038e1b8cf00fc))


### Documentation

* improve README first impression with problem/fix diagrams ([#109](https://github.com/mcpmux/mcp-mux/issues/109)) ([b15482b](https://github.com/mcpmux/mcp-mux/commit/b15482b32a016e3ca92753f26212f5827f744903))

## [0.1.1](https://github.com/mcpmux/mcp-mux/compare/v0.1.0...v0.1.1) (2026-02-16)


### Bug Fixes

* file-based keychain fallback for headless Linux/WSL ([#103](https://github.com/mcpmux/mcp-mux/issues/103)) ([9b60e0b](https://github.com/mcpmux/mcp-mux/commit/9b60e0bbe47a2318e7352efd3ba8b1888f393f38))
* stdio enable error UI state ([#104](https://github.com/mcpmux/mcp-mux/issues/104)) ([b4598e6](https://github.com/mcpmux/mcp-mux/commit/b4598e60e12d3389717fc2252bac8eb29e96f9c9))

## [0.1.0](https://github.com/mcpmux/mcp-mux/compare/v0.0.1...v0.1.0) (2026-02-16)

First public release of McpMux — the unified MCP gateway and manager for AI clients.

### Features

* Unified MCP gateway — configure servers once, connect every AI client through a single endpoint
* Encrypted credential storage via OS keychain (DPAPI, Keychain, Secret Service) with AES-256-GCM field-level encryption
* Spaces for organizing servers into workspaces with per-client access key authentication
* FeatureSet filtering — fine-grained control over tools, resources, and prompts per client
* OAuth 2.1 + PKCE with automatic token refresh for OAuth-enabled MCP servers
* Server discovery — browse and install from the community registry at mcpmux.com
* Streamable HTTP transport with SSE notifications
* Stdio transport with platform-specific process isolation
* Server connection logging with MCP protocol notifications and stderr capture
* Custom server configuration fields — environment variables, arguments, and headers
* Default values for server input definitions
* McpMux-branded OAuth authorization pages
* System tray with autostart on login
* Built-in auto-updater with signed releases
* Cross-platform installers — Windows (NSIS), macOS (DMG via Homebrew), Linux (APT + AppImage + .deb)
