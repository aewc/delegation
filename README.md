# delegation

IC supports the delegation call, that is to say:
* Identity A sign a update call request, and the msg.caller in canister will get the Principal of A.
* Identity A sign a delegation, such as the der encode public key of temp Identity B, and use the delegation(Identity B) to sign a update call request, and the msg.caller in canister will get the Principal of A.

The delegation Identity now is supported by [forked agent-rs](https://github.com/flyq/agent-rs)

## how run 
1. generate the Secp256k1 identity:
use [keysmith](https://github.com/dfinity/keysmith):
```sh
$ keysmith generate 
# will generate a seed.txt

$ keysmith private-key
# will generate a identity.pem, which is Secp256k1 private key.

$ dfx identity import ecdsa01 identity.pem
```
2. generate the Ed25519 identity:
```sh
$ dfx identity new 1
```

3. change the identity key path in `main.rs`, only need change the 7 `<username>`.

4. 
```sh
cargo test -- --nocapture
running 1 test
ecdsa_01: ghzvr-jfjro-sn66o-ajqul-m3xcc-pxmny-wp57a-l6hb6-lfmor-gynil-zqe
ed25519_01: as74e-clvxo-uxwuz-5dn2d-mqv2o-cwziv-iiemi-ckej2-yfwu6-zmj5u-mae
delegation_01: as74e-clvxo-uxwuz-5dn2d-mqv2o-cwziv-iiemi-ckej2-yfwu6-zmj5u-mae
delegation_02: ghzvr-jfjro-sn66o-ajqul-m3xcc-pxmny-wp57a-l6hb6-lfmor-gynil-zqe
delegation_01 call: as74e-clvxo-uxwuz-5dn2d-mqv2o-cwziv-iiemi-ckej2-yfwu6-zmj5u-mae
delegation_02 call: ghzvr-jfjro-sn66o-ajqul-m3xcc-pxmny-wp57a-l6hb6-lfmor-gynil-zqe
test test_delegation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.52s
```

## how II works

### a test II on mainnet
The canister id of test ii in mainnet is in `vxmla-4aaaa-aaaah-aausa-cai`.
The subnet id of the canister is `gmq5v-hbozq-uui6y-o55wc-ihop3-562wb-3qspg-nnijg-npqp5-he3cj-3ae`.

1. use the dfx default identity to register ii in test key. So get the default's der public key:
    ```rs
        // modify the path to default identity. and it's BasicIdentity(ed25519), so argument is true.
        let mut delegation_identity = DelegationIdentity::from_pem_file(
            "/Users/<username>/.config/dfx/identity/default/identity.pem",
            true,
        )
        .expect("Could not read the key pair.");
    ```
    ```sh
    $ cargo run pubkey
    ```
    then register the user number:
    ```
    $ dfx identity use default
    $ dfx canister --network ic call vxmla-4aaaa-aaaah-aausa-cai create_challenge
    (
        record {
            png_base64 = "xxx";
            challenge_key = "socfnxweyc";
        },
    )
    
    # the chars will always be "a", because it is in test mod
    $ dfx canister --network ic call vxmla-4aaaa-aaaah-aausa-cai register '(record {pubkey=vec{48:nat8;86:nat8;48:nat8;...;}; alias=""; credential_id=null; purpose=variant{authentication}; key_type=variant{unknown};}, record {key="socfnxweyc";chars="a";})'
    (variant { registered = record { user_number = 10_000 : nat64 } })

    # the 10000, "test" will always be bkgvx-uu7f4-25q5e-r47pl-rxafv-qifyj-2ggq5-qrrc4-zaej4-gwkj3-mae
    $ dfx canister --network ic call internet_identity get_principal '(10000, "test")' 
    (principal "bkgvx-uu7f4-25q5e-r47pl-rxafv-qifyj-2ggq5-qrrc4-zaej4-gwkj3-mae")
    ```
2. get the session public key:
   we use the ecdsa01 identity generated before.
   ```rs
        // modify the path to ecdsa01 identity.
        let mut delegation_identity = DelegationIdentity::from_pem_file(
            "/Users/<username>/.config/dfx/identity/ecdsa01/identity.pem",
            false,
        )
        .expect("Could not read the key pair.");
    ```
    ```sh
    $ cargo run public
    [48, 86, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 10, 3, 66, 0, 4, 14, 38, 225, 165, 197, 77, 164, 201, 41, 153, 7, 193, 131, 254, 196, 139, 194, 42, 49, 54, 111, 55, 16, 154, 31, 85, 119, 68, 189, 150, 95, 172, 141, 189, 210, 246, 206, 172, 200, 86, 79, 168, 156, 240, 80, 84, 167, 5, 173, 106, 27, 53, 176, 52, 168, 240, 207, 73, 101, 55, 45, 6, 15, 244]
    ```
3. Get the delegation. Using the user number 10000, and assuming FrontendHostname is “test”, the temporarily generated session public key is above, max_time_to_live is null to use the default.
```sh
$ dfx canister --network ic call vxmla-4aaaa-aaaah-aausa-cai prepare_delegation '(10000:nat64, "test", vec {48:nat8;86:nat8;48:nat8;16:nat8;6:nat8;7:nat8;42:nat8;134:nat8;72:nat8;206:nat8;61:nat8;2:nat8;1:nat8;6:nat8;5:nat8;43:nat8;129:nat8;4:nat8;0:nat8;10:nat8;3:nat8;66:nat8;0:nat8;4:nat8;14:nat8;38:nat8;225:nat8;165:nat8;197:nat8;77:nat8;164:nat8;201:nat8;41:nat8;153:nat8;7:nat8;193:nat8;131:nat8;254:nat8;196:nat8;139:nat8;194:nat8;42:nat8;49:nat8;54:nat8;111:nat8;55:nat8;16:nat8;154:nat8;31:nat8;85:nat8;119:nat8;68:nat8;189:nat8;150:nat8;95:nat8;172:nat8;141:nat8;189:nat8;210:nat8;246:nat8;206:nat8;172:nat8;200:nat8;86:nat8;79:nat8;168:nat8;156:nat8;240:nat8;80:nat8;84:nat8;167:nat8;5:nat8;173:nat8;106:nat8;27:nat8;53:nat8;176:nat8;52:nat8;168:nat8;240:nat8;207:nat8;73:nat8;101:nat8;55:nat8;45:nat8;6:nat8;15:nat8;244:nat8}, null)'
(
blob "0<0\0c\06\0a+\06\01\04\01\83\b8C\01\02\03,\00\0a\00\00\00\00\00\e0\05$\01\01\8b\aad\86yh+o\16z\e5\90\10S6z\09v\ed\c7\8d\f0\c8\27d\7f\86\96e\eb\9f(",
1_654_695_302_762_877_084 : nat64,
)

# 303c300c060a2b0601040183b8430102032c000a0000000000e0052401018baa648679682b6f167ae5901053367a0976edc78df0c827647f869665eb9f28 
# 1_654_695_302_762_877_084

dfx canister --network ic call vxmla-4aaaa-aaaah-aausa-cai get_delegation '(10000:nat64, "test", vec {48:nat8;86:nat8;48:nat8;16:nat8;6:nat8;7:nat8;42:nat8;134:nat8;72:nat8;206:nat8;61:nat8;2:nat8;1:nat8;6:nat8;5:nat8;43:nat8;129:nat8;4:nat8;0:nat8;10:nat8;3:nat8;66:nat8;0:nat8;4:nat8;14:nat8;38:nat8;225:nat8;165:nat8;197:nat8;77:nat8;164:nat8;201:nat8;41:nat8;153:nat8;7:nat8;193:nat8;131:nat8;254:nat8;196:nat8;139:nat8;194:nat8;42:nat8;49:nat8;54:nat8;111:nat8;55:nat8;16:nat8;154:nat8;31:nat8;85:nat8;119:nat8;68:nat8;189:nat8;150:nat8;95:nat8;172:nat8;141:nat8;189:nat8;210:nat8;246:nat8;206:nat8;172:nat8;200:nat8;86:nat8;79:nat8;168:nat8;156:nat8;240:nat8;80:nat8;84:nat8;167:nat8;5:nat8;173:nat8;106:nat8;27:nat8;53:nat8;176:nat8;52:nat8;168:nat8;240:nat8;207:nat8;73:nat8;101:nat8;55:nat8;45:nat8;6:nat8;15:nat8;244:nat8}, 1_654_695_302_762_877_084: nat64)'
(
  variant {
    signed_delegation = record {
      signature = blob "\d9\d9\f7\a2kcertificateY\05\ee\d9\d9\f7\a3dtree\83\01\83\01\83\01\83\02Hcanister\83\01\83\01\82\04X y\d8\bc\a8P\bd\1e\ac\b6o\05[@,O\b2bf\b3#\83\c4\a6\0d\bb\9d\1aF\f1\b47\f5\83\01\83\01\83\01\82\04X \abk}o;l\f0;\bc\b3v\c0\cc\0f0J\d0N\ef\den\97\a8l\1a\e7/|a\cd\cdO\83\01\83\01\82\04X \9f\fa\f4\0c\abz\f7\c8\86A\ae\01H\ea\db\a7874\e5\9d\0c\b5{1\f4i<\0c)\adw\83\01\82\04X I\84G\1a\a8\e8>\925\08t)4\9c\02\091\c8\8f\d3|\ed\c7\b4\af,4\07nk\19\ac\83\01\83\01\83\01\83\01\83\02J\00\00\00\00\00\e0\05$\01\01\83\01\83\01\83\01\83\02Ncertified_data\82\03X 5\ca\e1\8c2\9c\e3\d1\f4M\f1\f8\9fS\d4\ad\90\e5\9fp\bc(\17T!\a4\12\d4L[\c9\d3\82\04X \83\c5k\f1M\de=(\def\c6\92\b5\fc\9d\97\e9\dd\98[j\d7+\0f\e6\f8N\8a\8d\f3\dc\b2\82\04X \cas\a0s\11\de\dc\8a\e0<2\ccc\f6\9a~\15,\cdP%\b7\15\f2\f5\edW\d4\b4\e9\a5\07\82\04X e&ZCL\db\f8\04\27\f4\e9\f1Ha\f96\d4\97\0b@\f1\95#\da\80\94\22\b0\e7#\c6t\82\04X yaG\c4\fe\d4k\eds Y}\c74\b1\02\60\0dQN\f8\a0|\bfcj\d9~V\e7m\c6\82\04X \f0\b9\89Pq][,M\c3\a1\f8\19?\07Z\ba\9f\b6\9e\a0Jn\27\f6z\0a\e0{\f6Z\d5\82\04X \9at\db\d9\b3$B\eb\ca.\f4\8e\d9].\60u\1c\9a\86\ac*k\d5\f7\b2\f0\ecXIL\c8\82\04X \d5\a9\9c\e8\91\fd\f0Z\0d\ca\80\17\c6L86\f8\81\ce\97\bd\b7H\1d\8a\9b\b7Z&\8c\eb\80\82\04X \ceHL\ff*LT\ee\fe\b8\17\8e\f9\c7\8cC\bd\e9Nd\c5\cf\ac\0a\afq5\81\c03/O\82\04X \13S\993\efXPx\90W\b5\ddx\f2\9fP\17z\a0\8e\83\a6cK\ea\f6\ad\ea%R\b1\85\82\04X \bb\b2\c4\d8I\a00>(\133\9d\cb#c93\94k\06\0d\9a>\a0\02P\f9\de\ea h\a9\82\04X (-D\0e\bd\0bB[\e1\b8\9b\c1\b6C\9e\de\90;o\15\1bY\d8\ce\85E\90\9ch\e7\fd\bc\82\04X \9a\e9\b7\c3\c39\c7\e1\ba\60\bf\a0\a0~x\12\03*\acK\95f\8c\ee\60\92\de\0d\ba\81\e8\9f\82\04X  \5c\e4\80\a9)\ed\d7\e3Y\d08\954\b6\86.N\fd\af\d36\a4\b1\1bz\22\a9\98\a6\c9P\83\01\82\04X N\0b\0a\efI\a9\22\ca\f99\8d\9a\f0Z\a4\c5\ac\f2T\fc.\00b\95LMi\c8|9\91\d3\83\02Dtime\82\03I\be\e8\a1\89\db\e4\a9\fb\16isignatureX0\83\ea\0b}\7f*\b8\f6\0b\e5\08\c1\02q.\b0\8e\d2\0d\d0\dc\87\b0\88!:\d5\af\01oh o~\81i\d8eA\04V\fe\90\08\c9:Q\b9jdelegation\a2isubnet_idX\1d.\cc)D{\0e\efl$\1d\cf\df}\ab\07p\93\cc\d6\a1&k\e0\fe\9c\9b\12v\02kcertificateY\02W\d9\d9\f7\a2dtree\83\01\82\04X \bc\cd\c9\fe\e6\d2#\c9\9f\b3%3\e9@\0e\03\fd\9b\c8\a7\e2Dd\ca\8a\af\03KMR\d6\e0\83\01\83\02Fsubnet\83\01\83\01\83\01\82\04X \82\db\90=w:\9f \c0lUY\fe\ce\d6\e1\aa\fe\b3(\ef\ad\1aH\b31c\a1\07{\86\5c\83\01\83\01\83\01\83\02X\1d.\cc)D{\0e\efl$\1d\cf\df}\ab\07p\93\cc\d6\a1&k\e0\fe\9c\9b\12v\02\83\01\83\02Ocanister_ranges\82\03X\1b\d9\d9\f7\81\82J\00\00\00\00\00\e0\00\00\01\01J\00\00\00\00\00\ef\ff\ff\01\01\83\02Jpublic_key\82\03X\850\81\820\1d\06\0d+\06\01\04\01\82\dc|\05\03\01\02\01\06\0c+\06\01\04\01\82\dc|\05\03\02\01\03a\00\91T\1c\dc{e\c4\82\82\86\c9\11\60-\948\deVI\d6\98\b6\0f\c0j\ecsX\93\95\d0\bc\a7\17FRN\d2\ff\17\b2\c8\da\9f\bc\89\7f\0f\07\a4\0b Hq\b6\fe\96\d4^\f1\0bQ\d1\f1\d50\d0g\9a]\b8-\e9i)\80_\a1|sy\94\eb\cc#\12\d2\a2[\d9GG\ec\f8\f3K\82\04X 6\97}.\b5x\1a0\f3\92\aaI\b6\8a\99\e7R\e3\f1\80\e7\d6\c6]\c1\15[\ac\27 \96\03\82\04X p\ff\c8\b0t\ec?\16\c6<N\f6{\ff\fa\08o\81\ab\d7\1c\92\ca+\fbX\a0\fb_o\9a\18\82\04X \df\a7\dcP\19\a2\a5\ff\bf\609]\14\15\0b\0f\cbb0\14\84\db\e1\e2Z\e9Xn\80j \d2\82\04X \cc@g\f3\c0\90\95\ec\60\cd\de\d5\0b\f4C\ad \0b\aa\b2\c4\ef\d7\ce\0a}\00\be\97\b8v>\82\04X -\85k\ba{l\80\17\1c\e8\e5\d1!\bb~DP\b3VZdM\915\e9\feX\84(\1c\1f\b9\83\02Dtime\82\03I\83\a8\ca\be\95\cc\a0\fa\16isignatureX0\ab8\ef\e6\ea)\e420\fe1{vS3\b6t\df/\17\0el\c7\c6*T\da\b9)n\a7b\f2\ef\8c\b5\02~no\ac\88\e5\e2\18\e3\95\c2dtree\83\01\82\04X \d4\d5|?T\96e\afD\8ay\d7\ba\c5\b4v\aa\f5\e6\0d\e2A\27 \daMd\fb@\8dvi\83\02Csig\83\02X \06^uC\b7\8a\ab\09B\a7\b3\8d\f8\929\d03\d4\d1\0cQ\b1\eaD\c5H\ad\ab\a5\b1\9b\89\83\02X \ed\b5\a4\97?1>\cc\a4\8dA\c2_\a8\02\b7\84\b2\a1\db\af\7f\dc_>\00\d9s\9e\d1\88\15\82\03@";
      delegation = record {
        pubkey = blob "0V0\10\06\07*\86H\ce=\02\01\06\05+\81\04\00\0a\03B\00\04\0e&\e1\a5\c5M\a4\c9)\99\07\c1\83\fe\c4\8b\c2*16o7\10\9a\1fUwD\bd\96_\ac\8d\bd\d2\f6\ce\ac\c8VO\a8\9c\f0PT\a7\05\adj\1b5\b04\a8\f0\cfIe7-\06\0f\f4";
        targets = null;
        expiration = 1_654_695_302_762_877_084 : nat64;
      };
    }
  },
)

# d9d9f7a26b63657274696669636174655905eed9d9f7a3647472656583018301830183024863616e6973746572830183018204582079d8bca850bd1eacb66f055b402c4fb26266b32383c4a60dbb9d1a46f1b437f583018301830182045820ab6b7d6f3b6cf03bbcb376c0cc0f304ad04eefde6e97a86c1ae72f7c61cdcd4f83018301820458209ffaf40cab7af7c88641ae0148eadba7383734e59d0cb57b31f4693c0c29ad778301820458204984471aa8e83e9235087429349c020931c88fd37cedc7b4af2c34076e6b19ac830183018301830183024a0000000000e00524010183018301830183024e6365727469666965645f646174618203582035cae18c329ce3d1f44df1f89f53d4ad90e59f70bc28175421a412d44c5bc9d38204582083c56bf14dde3d28de66c692b5fc9d97e9dd985b6ad72b0fe6f84e8a8df3dcb282045820ca73a07311dedc8ae03c32cc63f69a7e152ccd5025b715f2f5ed57d4b4e9a5078204582065265a434cdbf80427f4e9f14861f936d4970b40f19523da809422b0e723c67482045820796147c4fed46bed7320597dc734b102600d514ef8a07cbf636ad97e56e76dc682045820f0b98950715d5b2c4dc3a1f8193f075aba9fb69ea04a6e27f67a0ae07bf65ad5820458209a74dbd9b32442ebca2ef48ed95d2e60751c9a86ac2a6bd5f7b2f0ec58494cc882045820d5a99ce891fdf05a0dca8017c64c3836f881ce97bdb7481d8a9bb75a268ceb8082045820ce484cff2a4c54eefeb8178ef9c78c43bde94e64c5cfac0aaf713581c0332f4f8204582013539933ef5850789057b5dd78f29f50177aa08e83a6634beaf6adea2552b18582045820bbb2c4d849a0303e2813339dcb23633933946b060d9a3ea00250f9deea2068a982045820282d440ebd0b425be1b89bc1b6439ede903b6f151b59d8ce8545909c68e7fdbc820458209ae9b7c3c339c7e1ba60bfa0a07e7812032aac4b95668cee6092de0dba81e89f82045820205ce480a929edd7e359d0389534b6862e4efdafd336a4b11b7a22a998a6c9508301820458204e0b0aef49a922caf9398d9af05aa4c5acf254fc2e0062954c4d69c87c3991d383024474696d65820349bee8a189dbe4a9fb16697369676e6174757265583083ea0b7d7f2ab8f60be508c102712eb08ed20dd0dc87b088213ad5af016f68206f7e8169d865410456fe9008c93a51b96a64656c65676174696f6ea2697375626e65745f6964581d2ecc29447b0eef6c241dcfdf7dab077093ccd6a1266be0fe9c9b1276026b6365727469666963617465590257d9d9f7a26474726565830182045820bccdc9fee6d223c99fb32533e9400e03fd9bc8a7e24464ca8aaf034b4d52d6e083018302467375626e65748301830183018204582082db903d773a9f20c06c5559feced6e1aafeb328efad1a48b33163a1077b865c8301830183018302581d2ecc29447b0eef6c241dcfdf7dab077093ccd6a1266be0fe9c9b127602830183024f63616e69737465725f72616e6765738203581bd9d9f781824a0000000000e0000001014a0000000000efffff010183024a7075626c69635f6b657982035885308182301d060d2b0601040182dc7c0503010201060c2b0601040182dc7c0503020103610091541cdc7b65c4828286c911602d9438de5649d698b60fc06aec73589395d0bca71746524ed2ff17b2c8da9fbc897f0f07a40b204871b6fe96d45ef10b51d1f1d530d0679a5db82de96929805fa17c737994ebcc2312d2a25bd94747ecf8f34b8204582036977d2eb5781a30f392aa49b68a99e752e3f180e7d6c65dc1155bac272096038204582070ffc8b074ec3f16c63c4ef67bfffa086f81abd71c92ca2bfb58a0fb5f6f9a1882045820dfa7dc5019a2a5ffbf60395d14150b0fcb62301484dbe1e25ae9586e806a20d282045820cc4067f3c09095ec60cdded50bf443ad200baab2c4efd7ce0a7d00be97b8763e820458202d856bba7b6c80171ce8e5d121bb7e4450b3565a644d9135e9fe5884281c1fb983024474696d6582034983a8cabe95cca0fa16697369676e61747572655830ab38efe6ea29e43230fe317b765333b674df2f170e6cc7c62a54dab9296ea762f2ef8cb5027e6e6fac88e5e218e395c26474726565830182045820d4d57c3f549665af448a79d7bac5b476aaf5e60de2412720da4d64fb408d766983024373696783025820065e7543b78aab0942a7b38df89239d033d4d10c51b1ea44c548adaba5b19b8983025820edb5a4973f313ecca48d41c25fa802b784b2a1dbaf7fdc5f3e00d9739ed18815820340
```
4. use the delegation to call whoami
```sh
# cargo run der_public_key expiration signature
$ cargo run 303c300c060a2b0601040183b8430102032c000a0000000000e0052401018baa648679682b6f167ae5901053367a0976edc78df0c827647f869665eb9f28 1654695302762877084 d9d9f7a26b63657274696669636174655905eed9d9f7a3647472656583018301830183024863616e6973746572830183018204582079d8bca850bd1eacb66f055b402c4fb26266b32383c4a60dbb9d1a46f1b437f583018301830182045820ab6b7d6f3b6cf03bbcb376c0cc0f304ad04eefde6e97a86c1ae72f7c61cdcd4f83018301820458209ffaf40cab7af7c88641ae0148eadba7383734e59d0cb57b31f4693c0c29ad778301820458204984471aa8e83e9235087429349c020931c88fd37cedc7b4af2c34076e6b19ac830183018301830183024a0000000000e00524010183018301830183024e6365727469666965645f646174618203582035cae18c329ce3d1f44df1f89f53d4ad90e59f70bc28175421a412d44c5bc9d38204582083c56bf14dde3d28de66c692b5fc9d97e9dd985b6ad72b0fe6f84e8a8df3dcb282045820ca73a07311dedc8ae03c32cc63f69a7e152ccd5025b715f2f5ed57d4b4e9a5078204582065265a434cdbf80427f4e9f14861f936d4970b40f19523da809422b0e723c67482045820796147c4fed46bed7320597dc734b102600d514ef8a07cbf636ad97e56e76dc682045820f0b98950715d5b2c4dc3a1f8193f075aba9fb69ea04a6e27f67a0ae07bf65ad5820458209a74dbd9b32442ebca2ef48ed95d2e60751c9a86ac2a6bd5f7b2f0ec58494cc882045820d5a99ce891fdf05a0dca8017c64c3836f881ce97bdb7481d8a9bb75a268ceb8082045820ce484cff2a4c54eefeb8178ef9c78c43bde94e64c5cfac0aaf713581c0332f4f8204582013539933ef5850789057b5dd78f29f50177aa08e83a6634beaf6adea2552b18582045820bbb2c4d849a0303e2813339dcb23633933946b060d9a3ea00250f9deea2068a982045820282d440ebd0b425be1b89bc1b6439ede903b6f151b59d8ce8545909c68e7fdbc820458209ae9b7c3c339c7e1ba60bfa0a07e7812032aac4b95668cee6092de0dba81e89f82045820205ce480a929edd7e359d0389534b6862e4efdafd336a4b11b7a22a998a6c9508301820458204e0b0aef49a922caf9398d9af05aa4c5acf254fc2e0062954c4d69c87c3991d383024474696d65820349bee8a189dbe4a9fb16697369676e6174757265583083ea0b7d7f2ab8f60be508c102712eb08ed20dd0dc87b088213ad5af016f68206f7e8169d865410456fe9008c93a51b96a64656c65676174696f6ea2697375626e65745f6964581d2ecc29447b0eef6c241dcfdf7dab077093ccd6a1266be0fe9c9b1276026b6365727469666963617465590257d9d9f7a26474726565830182045820bccdc9fee6d223c99fb32533e9400e03fd9bc8a7e24464ca8aaf034b4d52d6e083018302467375626e65748301830183018204582082db903d773a9f20c06c5559feced6e1aafeb328efad1a48b33163a1077b865c8301830183018302581d2ecc29447b0eef6c241dcfdf7dab077093ccd6a1266be0fe9c9b127602830183024f63616e69737465725f72616e6765738203581bd9d9f781824a0000000000e0000001014a0000000000efffff010183024a7075626c69635f6b657982035885308182301d060d2b0601040182dc7c0503010201060c2b0601040182dc7c0503020103610091541cdc7b65c4828286c911602d9438de5649d698b60fc06aec73589395d0bca71746524ed2ff17b2c8da9fbc897f0f07a40b204871b6fe96d45ef10b51d1f1d530d0679a5db82de96929805fa17c737994ebcc2312d2a25bd94747ecf8f34b8204582036977d2eb5781a30f392aa49b68a99e752e3f180e7d6c65dc1155bac272096038204582070ffc8b074ec3f16c63c4ef67bfffa086f81abd71c92ca2bfb58a0fb5f6f9a1882045820dfa7dc5019a2a5ffbf60395d14150b0fcb62301484dbe1e25ae9586e806a20d282045820cc4067f3c09095ec60cdded50bf443ad200baab2c4efd7ce0a7d00be97b8763e820458202d856bba7b6c80171ce8e5d121bb7e4450b3565a644d9135e9fe5884281c1fb983024474696d6582034983a8cabe95cca0fa16697369676e61747572655830ab38efe6ea29e43230fe317b765333b674df2f170e6cc7c62a54dab9296ea762f2ef8cb5027e6e6fac88e5e218e395c26474726565830182045820d4d57c3f549665af448a79d7bac5b476aaf5e60de2412720da4d64fb408d766983024373696783025820065e7543b78aab0942a7b38df89239d033d4d10c51b1ea44c548adaba5b19b8983025820edb5a4973f313ecca48d41c25fa802b784b2a1dbaf7fdc5f3e00d9739ed18815820340
bkgvx-uu7f4-25q5e-r47pl-rxafv-qifyj-2ggq5-qrrc4-zaej4-gwkj3-mae
bkgvx-uu7f4-25q5e-r47pl-rxafv-qifyj-2ggq5-qrrc4-zaej4-gwkj3-mae
```
and the result is the same to `dfx canister --network ic call internet_identity get_principal '(10000, "test")'`

Here the II used [canister signature/iccsa](https://github.com/dfinity/ic/blob/d79bd99f56/rs/crypto/internal/crypto_lib/basic_sig/iccsa/src/api.rs), and the der_encoded_public_key is not generated from the real public key, generated by Hash, and the signature is a bls signature, which sign the temp identity. temp identity generated by local pem file.
