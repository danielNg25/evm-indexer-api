use alloy::primitives::{Address, U256};
use anyhow::Result;

/// Chain-specific V2 factory addresses and their corresponding fees
pub mod factories {
    pub mod ethereum {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // Uniswap V2 - 0.3% fee
            ("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f", 3000),
            // Sushiswap - 0.3% fee
            ("0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac", 3000),
        ];
    }

    pub mod story {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // PIPERX V2 - 0.3% fee
            ("0x6D3e2f58954bf4E1d0C4bA26a85a1b49b2e244C6", 3000),
            // PIPERX V2 - 0.3% fee
            ("0xEeE400Eabfba8F60f4e6B351D8577394BeB972CD", 3000),
            // PIPERX V2 - 0.3% fee
            ("0x13e2362300a7733115d08edbf69214e811a0e262", 3000),
        ];
    }

    pub mod flare {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // BlazeSwap V2 - 0.3% fee
            ("0x440602f459d7dd500a74528003e6a20a46d6e2a6", 3000),
            // BlazeSwap V2 - 0.25% fee
            ("0xC1EdDCb8A8C5e5d6809D06C304BfBa99FAa16574", 3000),
            // FlareSwap V2 - 0.3% fee
            ("0x16b619B04c961E8f4F06C10B42FDAbb328980A89", 3000),
            // FlareSwap V2 - 0.3% fee
            ("0x3963059957eF80BAcb0F2bFeDBB2B97e47aC4475", 3000),
            // Enosys DEX V2 - 0.3% fee
            ("0x28b70f6Ed97429E40FE9a9CD3EB8E86BCBA11dd4", 3000),
            // Pangolin V2 - 0.3% fee
            ("0xFf1B852A0582BF87E69FaD114560595FC5cF1212", 3000),
            // Pangolin V2 - 0.3% fee
            ("0xbfe13753156b9c6b2818FB45ff3D2392ea43d79A", 3000),
            // Xenos V2 - 0.25% fee
            ("0x7D8A26Bd4d2580B4Be2df0a051F3bB5f218B0c3A", 2500),
        ];
    }

    pub mod soneium {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x21B7668ADFe8Cc1F51011856A181F4EBee86d231", 2000),
            ("0x4f0c1b4c6FdF983f2d385Cf24DcbC8c68f345E40", 5000),
            ("0x82d2d0aAE77967d42ACf4F30B53e2de0055338De", 5000),
            ("0x97FeBbC2AdBD5644ba22736E962564B23F5828CE", 3000),
            ("0xC3d4fA777308412CbA0520c4034Ad3567de852dF", 3000),
            ("0xdb5D9562C80AEab3aeaED35ecaAe40Fd8DC9a4c8", 3000),
        ];
    }

    pub mod fraxtal {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0xE30521fe7f3bEB6Ad556887b50739d6C7CA667E6", 3000),
            ("0x7a07D606c87b7251c2953A30Fa445d8c5F856C7A", 3000),
            ("0x0000000000000000000000000000000000000000", 5000),
        ];
    }

    pub mod bsc {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // PancakeSwap V2 - 0.25% fee
            ("0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73", 2500),
            // Biswap - 0.10% fee
            ("0x858E3312ed3A876947EA49d572A7C42DE08af7EE", 2000),
            // PancakeSwap V2 - 0.20% fee
            ("0x0841BD0B734E4F5853f0dD8d7Ea041c241fb0Da6", 2000),
            // PancakeSwap V2 - 0.20% fee
            ("0xBCfCcbde45cE874adCB698cC183deBcF17952812", 2000),
            // BabySwap - 0.20% fee
            ("0x86407bEa2078ea5f5EB5A52B2caA963bC1F889Da", 2000),
            // FstSwap - 0.30% fee
            ("0x9A272d734c5a0d7d84E0a892e891a553e8066dce", 3000),
            // NomiSwap - 0.10% fee
            ("0xd6715A8be3944ec72738F0BFDC739d48C3c29349", 1000),
            // SwapV2 - 0.30% fee
            ("0x3CD1C46068dAEa5Ebb0d3f55F6915B10648062B8", 3000),
            // WaultSwap - 0.20% fee
            ("0xB42E3FE71b7E0673335b3331B3e1053BD9822570", 2000),
            // Uniswap V2 - 0.30% fee
            ("0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6", 3000),
            // SquadSwap - 0.20% fee
            ("0x918Adf1f2C03b244823Cd712E010B6e3CD653DbA", 2000),
            // BabyDoge - 0.00% fee
            ("0x4693B62E5fc9c0a45F89D62e6300a03C85f43137", 0),
            // Definix - 0.20% fee
            ("0x43eBb0cb9bD53A3Ed928Dd662095aCE1cef92D19", 2000),
            // OrionPool V2 - 0.30% fee
            ("0xE52cCf7B6cE4817449F2E6fA7efD7B567803E4b4", 3000),
            // Swap - 0.30% fee
            ("0x73D9F93D53505cB8C4c7f952ae42450d9E859D10", 3000),
            // PancakeSwap V2 - 0.25% fee
            ("0x35b4B2Fb6D3156E44A2CdD9006f81B0371B3D808", 2500),
            // Uniswap V2 - 0.30% fee
            ("0xc35DADB65012eC5796536bD9864eD8773aBc74C4", 3000),
            // PancakeSwap V2 - 0.20% fee
            ("0xf0bc2E21a76513aa7CC2730C7A1D6deE0790751f", 2000),
            // AlitaSwap - 0.20% fee
            ("0xC7a506ab3ac668EAb6bF9eCf971433D6CFeF05D9", 2000),
            // CoinSwap - 0.20% fee
            ("0xC2D8d27F3196D9989aBf366230a47384010440c0", 2000),
            // CheeseSwap - 0.20% fee
            ("0xdd538E4Fd1b69B7863E1F741213276A6Cf1EfB3B", 2000),
            // PancakeSwap V2 - 0.25% fee
            ("0x877Fe7F4e22e21bE397Cd9364fAFd4aF4E15Edb6", 2500),
            // PancakeSwap V2 - 0.25% fee
            ("0xcb4Ee9910811EdB5fF3fe0e3CE3A8cEd25E24079", 2500),
            // PancakeSwap V2 - 0.2% fee
            ("0x71f843BD057d2eE39AE52186a33c3aFD1124805A", 2500),
            // PancakeSwap V2 - 0.2% fee
            ("0x957d1361F1929Daa61c7d41C8561559Cf58b13f3", 1000),
            // PancakeSwap V2 - 0.2% fee
            ("0x670f55c6284c629c23baE99F585e3f17E8b9FC31", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0x137f34dF5bcDB30f5E858FC77CB7Ab60f8F7a09a", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0xae52c26976E56e9f8829396489A4b7FfEbe8aAE9", 1000),
            // PancakeSwap V2 - 0.2% fee
            ("0x3657952d7bA5A0A4799809b5B6fdfF9ec5B46293", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0xF2Fb1b5Be475E7E1b3C31082C958e781f73a1712", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0xCe8fd65646F2a2a897755A1188C04aCe94D2B8D0", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0xA78AAc0C0551ab3470F40ff5A382f0CDbFA31B7b", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0x1D9F43a6195054313ac1aE423B1f810f593b6ac1", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0x137f34dF5bcDB30f5E858FC77CB7Ab60f8F7a09a", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0x94b4188D143b9dD6bd7083aE38A461FcC6AAd07E", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0x81Bb3E7b2448786a82FEC9fe49e311af683F6723", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0x7ab906d0ff39AADa772Ec95829fb6A048f19d531", 0000),
            // PancakeSwap V2 - 0.2% fee
            ("0x79C342FddBBF376cA6B4EFAc7aaA457D6063F8Cb", 1700),
            // PancakeSwap V2 - 0.2% fee
            ("0xa053582601214FEb3778031a002135cbBB7DBa18", 2500),
            // PancakeSwap V2 - 0.2% fee
            ("0x0eb58E5c8aA63314ff5547289185cC4583DfCBD5", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0x4a3B76860C1b76f0403025485DE7bfa1F08C48fD", 2500),
            // PancakeSwap V2 - 0.2% fee
            ("0x59DA12BDc470C8e85cA26661Ee3DCD9B85247C88", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0x1e895bFe59E3A5103e8B7dA3897d1F2391476f3c", 10000),
            // PancakeSwap V2 - 0.2% fee
            ("0xF238d267B3B1C85F2a95354251C20626bb7bc2A1", 860),
            // PancakeSwap V2 - 0.2% fee
            ("0x86A859773cf6df9C8117F20b0B950adA84e7644d", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0x787557689775Df6791c729014C78ABAC6Cb8F632", 1000),
            // PancakeSwap V2 - 0.2% fee
            ("0x97bCD9BB482144291D77ee53bFa99317A82066E8", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0x80f112CD8Ac529d6993090A0c9a04E01d495BfBf", 2500),
            // PancakeSwap V2 - 0.2% fee
            ("0x8BA1a4C24DE655136DEd68410e222cCA80d43444", 1000),
            // PancakeSwap V2 - 0.2% fee
            ("0x7F6AD1d60De7a908A28a25DBe961ABb68747cEB3", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0xAF3bA99201485E14472353a10dd7392845826314", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0xaF042b1B77240063bc713B9357c39ABedec1b691", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0xDB984fd8371d07db9cBf4A48Eb9676b09B12161D", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0xDD3779945963a270652bc5bAfdC2b79B7e7428C8", 200),
            // PancakeSwap V2 - 0.2% fee
            ("0x4d05D0045df5562D6D52937e93De6Ec1FECDAd21", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0x4DcE5Bdb81B8D5EdB66cA1b8b2616A8E0Dd5f807", 2500),
            // PancakeSwap V2 - 0.2% fee
            ("0xd654CbF99F2907F06c88399AE123606121247D5C", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0x918d7e714243F7d9d463C37e106235dCde294ffC", 600),
            // PancakeSwap V2 - 0.2% fee
            ("0x20aB15EaAFB195DeE6a145e845a8e6066513357D", 2500),
            // PancakeSwap V2 - 0.2% fee
            ("0x3e708FdbE3ADA63fc94F8F61811196f1302137AD", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0xB9fA84912FF2383a617d8b433E926Adf0Dd3FEa1", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0x738B815eaDD06E0041b52B0C9d4F0d0D277B24bA", 3000),
            // PancakeSwap V2 - 0.2% fee
            ("0xCBac17919f7aad11E623Af4FeA98B10B84802eAc", 2000),
            // PancakeSwap V2 - 0.2% fee
            ("0x1Ba94C0851D96b2c0a01382Bf895B5b25361CcB2", 3000),
            // PancakeSwap V2 - 0.2% fee
        ];
    }

    pub mod polygon {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // Quickswap - 0.3% fee
            ("0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32", 3000),
        ];
    }

    pub mod bera {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // 0.3% fee
            ("0x5e705e184D233FF2A7cb1553793464a9d0C3028F", 3000),
            // 0.2% fee
            ("0x134147627c2dC9a0E589EC43d3F8866AAA0Bd1ba", 2000),
            // 0.5% fee - Dyorswap
            ("0x83ad0f601fAEE9d867e5f22fFDcd812885EC2f62", 5000),
        ];
    }

    pub mod kava {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // 0.3% fee
            ("0xD408a20f1213286fB3158a2bfBf5bFfAca8bF269", 3000),
            // 0.3% fee
            ("0xE8E917BC80A26CDacc9aA42C0F4965d2E1Fa52da", 3000),
            // 0.3% fee
            ("0xeEAbe2F15266B19f3aCF743E69105016277756Fb", 3000),
            // 0.3% fee
            ("0x4FD2c40c25Dd40e9Bf0CE8479bA384178b8671b5", 3000),
            // 0.3% fee
            ("0x8F1fD6Ed57B0806FF114135F5b50B5f76e9542F2", 3000),
            // 0.3% fee
            ("0xC012C4b3d253A8F22d5e4ADA67ea2236FF9778fc", 3000),
        ];
    }

    pub mod hbar {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // 0.3% fee
            ("0x0000000000000000000000000000000000103780", 3000),
            // 0.3% fee
            ("0x0000000000000000000000000000000000134224", 3000),
        ];
    }

    pub mod base {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // 0.3% fee
            ("0x420DD381b31aEf6683db6B902084cB0FFECe40Da", 3000),
        ];
    }

    pub mod core {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // 0.3% fee
            ("0xe0b8838e8d73ff1CA193E8cc2bC0Ebf7Cf86F620", 2000),
            ("0x326Ee96748E7DcC04BE1Ef8f4E4F6bdd54048932", 2500),
            ("0x1a34538D5371e9437780FaB1c923FA21a6facbaA", 3000),
            ("0x9E6d21E759A7A288b80eef94E4737D313D31c13f", 3000),
            ("0x6Edf8aecAA888896385d7fA19D2AA4eaff3C10D8", 2500),
            ("0x666666668DEAb6b4A627c97b1fBac629D2Da4795", 2500),
            ("0x3E723C7B6188E8Ef638DB9685Af45c7CB66f77B9", 3000),
            ("0xA1ADD165AED06D26fC1110b153ae17a5A5ae389e", 3000),
            ("0x74739487C28B14e732F9Ab755441Cc1dcB6C592d", 2500),
            ("0xab40A3d72a4305e9215aE0781205bd3e26E1cbcd", 2500),
            ("0xC90a6D83764825B2bbD32A7D3E577a501363EA20", 2500),
            ("0x300EB5D633c8154466aFC73E98328E882E3D843D", 3000),
            ("0xB45e53277a7e0F1D35f2a77160e91e25507f1763", 3000),
            ("0xb8b9a4d9beE1fB41b03edfa47640b1dadF49EDd2", 2500),
            ("0x91cE3Cf997CAD223654764b4338A92431997AFe9", 3000),
            ("0x771E49134e4b12132bA0bFE259E465b4307D5D7C", 2000),
            ("0xE66D650878E8Ff662DF3B4AbA9f6C421D1F766F6", 2500),
            ("0x229eeE4a12cdD1b5c49638368ee0A1c7F85d9aE0", 3000),
            ("0xfb6E605049b7D969719bf973A7685115Ff17327f", 2000),
            ("0x23556027Ad3C3e76160AcA51e8098C395a6d815C", 2500),
            ("0x97814a1F542aFe7fd02de53926621b0D40e8Ad6C", 5000),
            ("0xd1b0bE39549A0685579cEdE54dB4365F16CcdfBc", 2500),
            ("0x4FF315624D1E6C9E90d13889362B1BE39419F06f", 2000),
            ("0x7382D5A25A281A3442Abc6f03BdcC001BC6715f6", 3000),
            ("0x98016C2C8839D8aa4a5aE530748903e0bB432036", 3000),
            ("0x47582Fd7B5189A3B2066FC2F631c51776662e3E4", 2500),
            ("0x64879d7240eA8e7125Dd51F496fff0B7CA0e0B7c", 2500),
            ("0xfcb9Eb102B53893cdB80D29b2F37004e305eBc8b", 2500),
        ];
    }

    pub mod avalanche {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            // joeCall: 0.30% fee
            ("0x9ad6c38be94206ca50bb0d90783181662f0cfa10", 3000),
            // arenaCall: 0.00% fee
            ("0xF16784dcAf838a3e16bEF7711a62D12413c39BD1", 3000),
            // pangolinCall: 0.30% fee
            ("0xefa94de7a4656d787667c749f7e1223d71e9fd88", 3000),
            // uniswapV2Call: 0.30% fee
            ("0xc35dadb65012ec5796536bd9864ed8773abc74c4", 3000),
            // sicleCall: 0.30% fee
            ("0x9c60c867ce07a3c403e2598388673c10259ec768", 3000),
            // uniswapV2Call: 0.30% fee
            ("0xf77ca9b635898980fb219b4f4605c50e4ba58aff", 3000),
            // uniswapV2Call: 0.30% fee
            ("0x9e5a52f57b3038f1b8eee45f28b3c1967e22799c", 3000),
            // lydiaCall: 0.20% fee
            ("0xe0c1bb6df4851feeedc3e14bd509feaf428f7655", 2000),
            // uniswapV2Call: 0.30% fee
            ("0x8e6f4af0b6c26d16febdd6f28fa7c694bd49c6bf", 3000),
            // alligatorCall: 0.30% fee
            ("0xd9362aa8e0405c93299c573036e7fb4ec3be1240", 3000),
            // forwardCall: 0.20% fee
            ("0x2131bdb0e0b451bc1c5a53f2cbc80b16d43634fa", 2000),
            // uniswapV2Call: 0.10% fee
            ("0xa0fbfda09b8815dd42ddc70e4f9fe794257cd9b6", 1000),
            // elkCall: 0.30% fee
            ("0x091d35d7f63487909c863001ddca481c6de47091", 3000),
            // uniswapV2Call: 0.30% fee
            ("0x26b42c208d8a9d8737a2e5c9c57f4481484d4616", 3000),
            // yetiswapCall: 0.30% fee
            ("0x58c8cd291fa36130119e6deb9e520fbb6aca1c3a", 3000),
            // hook: 0.50% fee
            ("0xaaa16c016bf556fcd620328f0759252e29b1ab57", 5000),
            // hakuswapCall: 0.20% fee
            ("0x2db46feb38c57a6621bca4d97820e1fc1de40f41", 2000),
            // uniswapV2Call: 0.50% fee
            ("0x814EBF333BDaF1D2d364c22a1e2400a812f1F850", 20000),
            // canaryCall: 0.30% fee
            ("0xcfba329d49c24b70f3a8b9cc0853493d4645436b", 3000),
            // complusV2Call: 0.30% fee
            ("0x5c02e78a3969d0e64aa2cfa765acc1d671914ac0", 3000),
            // swapCall: 0.10% fee
            ("0xa98ea6356a316b44bf710d5f9b6b4ea0081409ef", 1000),
            // VaporDEXCall: 0.29% fee
            ("0xc009a670e2b02e21e7e75ae98e254f467f7ae257", 2900),
            // uniswapV2Call: 0.30% fee
            ("0x0c6a0061f9d0afb30152b8761a273786e51bec6d", 3000),
            // soulswapCall: 0.20% fee
            ("0x5bb2a9984de4a69c05c996f7ef09597ac8c9d63a", 2000),
            // hook: 0.20% fee
            ("0x634e02eb048eb1b5bddc0cfdc20d34503e9b362d", 2000),
            // uniswapV2Call: 0.30% fee
            ("0x16871f3c042a9b0467f8166dbe6cddc6ec557a74", 3000),
            // uniswapV2Call: 0.30% fee
            ("0x7cfd6f0fb0802db028d461ca25daa0ba863a1f45", 3000),
            // uniswapV2Call: 0.70% fee
            ("0x045d720873f0260e23da812501a7c5930e510aa4", 7000),
            // partyCall: 0.20% fee
            ("0x58a08bc28f3e8dab8fb2773d8f243bc740398b09", 2000),
            // uniswapV2Call: 0.30% fee
            ("0x7009b3619d5ee60d0665ba27cf85edf95fd8ad01", 3000),
            // contextCall: 0.30% fee
            ("0x9f0e80ac5e09dd1e37b40e8cdd749768fead43eb", 3000),
            // flashLiquidityCall: 0.30% fee
            ("0x6e553d5f028bd747a27e138fa3109570081a23ae", 3000),
            // pancakeCall: 0.50% fee
            ("0x042af448582d0a3ce3cfa5b65c2675e88610b18d", 5000),
            // StormCall: 0.25% fee
            ("0x03c51a75a94b1cd075d6686846405dbdafbde390", 2500),
            // AvaxPadSwapCall: 0.25% fee
            ("0x2ffa939c7db9d4b4278713add0154b70cb82aa82", 2500),
            // moeCall: 0.00% fee
            ("0x1051E74C859cc1e662C3AFa3F170103522A2e70f", 0),
            // pancakeCall: 0.50% fee
            ("0xd9f58f79bcdfb5cf5e7741eb14ca4060d32f2b21", 5000),
            // uniswapV2Call: 0.50% fee
            ("0xc7e37a28bb17edb59e99d5485dc8c51bc87ae699", 5000),
            // oliveCall: 0.20% fee
            ("0x4fe4d8b01a56706bc6cad26e8c59d0c7169976b3", 2000),
            // pancakeCall: 0.50% fee
            ("0xdc0bd72cdef330786bf6f331a6aca539c0bb4eab", 5000),
            // baguetteCall: 0.30% fee
            ("0x3587b8c0136c2c3605a9e5b03ab54da3e4044b50", 3000),
            // uniswapV2Call: 0.30% fee
            ("0x6ab0c582b8e25b5b575c2797c4bef3aa2827a58a", 3000),
            // pancakeCall: 0.25% fee
            ("0x2181B20a9aaE3C41Bbd7aDD59233Cc1B629a54eB", 2500),
            // uniswapV2Call: 0.50% fee
            ("0xa0bb8f9865f732c277d0c162249a4f6c157ae9d0", 5000),
            // uniswapV2Call: 0.20% fee
            ("0xe61a092225a6639844693626491200be1333d5cb", 2000),
            // ruggyCall: 0.20% fee
            ("0x9a89fa373186ecc1ccb3b9fe08335ffd9cdf35d8", 2000),
            // Call: 0.30% fee
            ("0xdfd34be29a8ffb58dea78bd7a6340b89ebeebbe2", 3000),
            // lpCall: 0.30% fee
            ("0x557ade9f0c89d07c396b19c4efac102e4008736e", 3000),
            // 4a639df4: 0.50% fee
            ("0xe357f7d5652004d41a8e9405a5454ec94173e3e7", 5000),
            // apexCall: 0.20% fee
            ("0x21cadeb92c8BbFBEF98c3098846f0999209C3A97", 2000),
            // viralswapCall: 0.50% fee
            ("0x71255b66e1977be3b5e427256495e811774729f6", 5000),
            // uniswapV2Call: 0.50% fee
            ("0x3bca0b7431f46050a99ec3b1b7bb710b3efd30dd", 5000),
            // uniswapV2Call: 0.50% fee
            ("0x7ab5ac142799b0a3b6f95c27a1f2149ebcf5287d", 5000),
            // unifiCall: 0.50% fee
            ("0x839547067bc885db205F5fA42dcFeEcDFf5A8530", 5000),
            // joeCall: 0.50% fee
            ("0x231df4d421f1f9e0aae9ba3634a87ebc87a09c39", 5000),
            // ovxCall: 0.30% fee
            ("0xE01cF83a89e8C32C0A9f91aCa7BfE554EBEE7141", 3000),
            // uniswapV2Call: 0.50% fee
            ("0x45c2c071b503e734b4f05634e57d6997d39534a7", 5000),
            // whaleswapCall: 0.25% fee
            ("0xabc26f8364cc0dd728ac5c23fa40886fda3dd121", 2500),
            // d66af394: 0.50% fee
            ("0x38a83a88e6d77576083fd755d7387779eb291792", 5000),
            // AzurSwapV2Call: 0.20% fee
            ("0x26ba4ce017bcd67e2ca9135bd58d3fc9050fc58f", 2000),
            // zeroCall: 0.50% fee
            ("0x2ef422f30cdb7c5f1f7267ab5cf567a88974b308", 5000),
            // hakuswapCall: 0.10% fee
            ("0x3a9c3398d7bfe6149a5580d901b1f57b1c7d3ec0", 1000),
            // sicleCall: 0.30% fee
            ("0xa22fff80baef689976c55dabb193becdf023b6b9", 3000),
            // uniswapV2Call: 0.30% fee
            ("0x8c7437a3a5882b197970d4351a9341fb9e0bfe39", 3000),
            // CandyManCall: 0.20% fee
            ("0xde105b2137045e8b7ea28ea8ab98ea1f859a6562", 2000),
            // otbSwapCall: 0.20% fee
            ("0xcdf5ebcfb2b9608ee81ff043100abbc45c9e4599", 2000),
            // uniswapV2Call: 0.30% fee
            ("0x87033126710cca6d51c1a6a4f8a13f42ef12e434", 3000),
            // uniswapV2Call: 0.20% fee
            ("0xdec9231b2492cce6ba01376e2cbd2bd821150e8c", 2000),
            // zswapCall: 0.25% fee
            ("0xcDE3F9e6D452be6d955B1C7AaAEE3cA397EAc469", 2500),
            // uniswapV2Call: 0.30% fee
            ("0x5ca135cb8527d76e932f34b5145575f9d8cbe08e", 3000),
            // icecreamCall: 0.00% fee
            ("0xac7b7eac8310170109301034b8fdb75eca4cc491", 2000),
        ];
    }

    pub mod ink {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x458C5d5B75ccBA22651D2C5b61cB1EA1e0b0f95D", 3000),
            ("0x6c86ab200661512fDBd27Da4Bb87dF15609A2806", 5000),
            ("0x63b54dBBD2DAbf89D5c536746e534711f6094199", 3000),
            ("0xfe57A6BA1951F69aE2Ed4abe23e0f095DF500C04", 3000),
            ("0xBD5B41358A6601924F1Fd708aF1535a671f530A9", 3000),
        ];
    }

    pub mod mantle {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x5bEf015CA9424A7C07B68490616a4C1F094BEdEc", 3000),
            ("0xE5020961fA51ffd3662CDf307dEf18F9a87Cce7c", 2000),
        ];
    }

    pub mod katana {
        pub const V2_FACTORIES: &[(&str, u32)] =
            &[("0x72D111b4d6f31B38919ae39779f570b747d6Acd9", 3000)];
    }

    pub mod pulse {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x1715a3e4a142d8b698131108995174f37aeba10d", 2900),
            ("0x29ea7545def87022badc76323f373ea1e707c523", 2900),
            ("0x5b9f077a77db37f3be0a5b5d31baeff4bc5c0bd7", 2900),
        ];
    }

    pub mod iotx {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0xda257cBe968202Dea212bBB65aB49f174Da58b9D", 3000),
            ("0xF96bE66DA0b9bC9DFD849827b4acfA7e8a6F3C42", 3000),
        ];
    }

    pub mod zetachain {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x9fd96203f7b22bCF72d9DCb40ff98302376cE09c", 3000),
            ("0x33d91116e0370970444B0281AB117e161fEbFcdD", 3000),
        ];
    }

    pub mod metis {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0xF38E7c7f8eA779e8A193B61f9155E6650CbAE095", 1500), // TODO
            ("0x70f51d68D16e8f9e418441280342BD43AC9Dff9f", 3000),
            ("0x2CdFB20205701FF01689461610C9F321D1d00F80", 2000),
            ("0xFA68bAAdBDCf014fA20bD1A4542967AE40Ddca53", 5000), // TODO
            ("0x3c4063B964B1b3bF229315fCc4df61a694B0aE84", 10),   // TODO
            ("0x580ED43F3BBa06555785C81c2957efCCa71f7483", 3000),
        ];
    }

    pub mod xdc {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x347D14b13a68457186b2450bb2a6c2Fd7B38352f", 3000),
            ("0x9fAb572F75008A42c6aF80b36Ab20C76a38ABc4B", 3000),
            ("0xA8334Aae58e5bDee692B26679c1817F9c42f8f51", 3000),
            ("0x9E6d21E759A7A288b80eef94E4737D313D31c13f", 3000),
            ("0xAf2977827a72e3CfE18104b0EDAF61Dd0689cd31", 3000),
            ("0x9c70B6B8e389b2C97090FFFA6bE3a13626ba3018", 3000),
            ("0x061715D0e7b91d436a4a57419d013dED490c264D", 3000),
        ];
    }

    pub mod zero {
        pub const V2_FACTORIES: &[(&str, u32)] =
            &[("0x1B4427e212475B12e62f0f142b8AfEf3BC18B559", 3000)];
    }

    pub mod chiliz {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0xE2918AA38088878546c1A18F2F9b1BC83297fdD3", 3000),
            ("0xA0BB8f9865f732C277d0C162249A4F6c157ae9D0", 3000),
            ("0xcF4A2be8Fe92fEe8e350AD8D876274749Ae0CBb1", 3000),
            ("0xBDd9c322Ecf401E09C9D2Dca3be46a7E45d48BB1", 3000),
        ];
    }

    pub mod lisk {
        pub const V2_FACTORIES: &[(&str, u32)] =
            &[("0x31832f2a97Fd20664D76Cc421207669b55CE4BC0", 3000)];
    }

    pub mod rootstock {
        pub const V2_FACTORIES: &[(&str, u32)] =
            &[("0xB45e53277a7e0F1D35f2a77160e91e25507f1763", 3000)];
    }

    pub mod etherlink {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x3eebf549D2d8839E387B63796327eE2C8f64A0C4", 2500),
            ("0x033eff22bC5Bd30c597e1fdE8Ca6fB1C1274C688", 2000),
        ];
    }

    pub mod sei {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x71f6b49ae1558357bBb5A6074f1143c46cBcA03d", 3000),
            ("0xd45dAff288075952822d5323F1d571e73435E929", 1800),
            ("0xAEbdA18889D6412E237e465cA25F5F346672A2eC", 3000),
        ];
    }

    pub mod somnia {
        pub const V2_FACTORIES: &[(&str, u32)] = &[
            ("0x6C4853C97b981Aa848C2b56F160a73a46b5DCCD4", 3000),
            ("0xaFd71143Fb155058e96527B07695D93223747ed1", 5000),
        ];
    }
}

/// Get the fee for a given V2 factory address
pub fn get_v2_factory_fee(factory_address: &Address) -> Result<U256> {
    let address_str = format!("{:#x}", factory_address);

    // Check all chains
    let factories = [
        factories::ethereum::V2_FACTORIES,
        factories::bsc::V2_FACTORIES,
        factories::polygon::V2_FACTORIES,
        factories::avalanche::V2_FACTORIES,
        factories::story::V2_FACTORIES,
        factories::flare::V2_FACTORIES,
        factories::soneium::V2_FACTORIES,
        factories::fraxtal::V2_FACTORIES,
        factories::bera::V2_FACTORIES,
        factories::core::V2_FACTORIES,
        factories::ink::V2_FACTORIES,
        factories::mantle::V2_FACTORIES,
        factories::katana::V2_FACTORIES,
        factories::pulse::V2_FACTORIES,
        factories::iotx::V2_FACTORIES,
        factories::zetachain::V2_FACTORIES,
        factories::metis::V2_FACTORIES,
        factories::xdc::V2_FACTORIES,
        factories::zero::V2_FACTORIES,
        factories::chiliz::V2_FACTORIES,
        factories::lisk::V2_FACTORIES,
        factories::rootstock::V2_FACTORIES,
        factories::etherlink::V2_FACTORIES,
        factories::sei::V2_FACTORIES,
        factories::somnia::V2_FACTORIES,
        factories::base::V2_FACTORIES,
    ];

    factories
        .iter()
        .flat_map(|&chain_factories| chain_factories.iter())
        .find(|(addr, _)| addr.eq_ignore_ascii_case(&address_str))
        .map(|(_, fee)| U256::from(*fee))
        .ok_or_else(|| anyhow::anyhow!("Unknown V2 factory address: {}", address_str))
}

pub fn default_factory_fee_by_chain_id(chain_id: u64, factory_address: &Address) -> Result<U256> {
    match chain_id {
        252 => Ok(U256::from(5000)),
        43114 => Ok(U256::from(5000)),
        2222 => Ok(U256::from(4000)),
        295 => Ok(U256::from(3000)),
        5000 => Ok(U256::from(3000)),
        4689 => Ok(U256::from(5000)),
        1135 => Ok(U256::from(3000)),
        1329 => Ok(U256::from(1800)),
        _ => Err(anyhow::anyhow!(
            "Unknown chain ID: {}, factory address: {}",
            chain_id,
            factory_address
        )),
    }
}
