MSG=$(cat <<EOF
{
  "create_site": {
    "name": "$1"
  }
}
EOF
)

starsd tx wasm execute $SITES "$MSG" \
  --amount 1000000000ustars \
  --gas-prices 0.025ustars --gas auto --gas-adjustment 1.9 \
  --from $TESTNET_KEY -b block -y -o json | jq .