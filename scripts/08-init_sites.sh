MSG=$(cat <<EOF
{
  "site_creation_fee": "100000000",
  "fee_account": $(starsd keys show $TESTNET_FEE_KEY | jq '.address'),
  "fair_burn_bps": 1000,
  "sg721_name_contract_addr": "$COLLECTION"
}
EOF
)

starsd tx wasm instantiate $SITES_CODE_ID "$MSG" --label "Sites" \
 --admin $ADMIN \
 --gas-prices 0.025ustars --gas auto --gas-adjustment 1.9 \
 --from $TESTNET_KEY -y -b block -o json | jq .
 
