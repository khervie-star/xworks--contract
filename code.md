txhash=B8DD668A60CE33D0B0BECC22D6C7817545FA5BD5EF4C6D512FB3F865C1DB65A3
codeId=220
txhash2=218564A455AE23C289BD40B574D1EAA2A8D1CC5F4CDAC58BC2A8C79DD4B4A92C


CA1=xion1833k7wd0kctat840yckv946gc2w074lkceenlawzpeynhd7c204qx592a8


txhash1=3EC5C34B1AB9CE278E73CC23B0836D40AE3252B6C3C38174A0D64A3CE2F14639
codeid2=254


xiond tx wasm instantiate $CODE_ID "$MSG" \
  --from $WALLET \
  --label "Xworks" \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y --no-admin \
  --chain-id xion-testnet-2 \
  --node https://rpc.xion-testnet-2.burnt.com:443

  MSG='{"admin": "xion1nvx9jzdgddnk4vajjhruz3ta20l656459auntt"}'
