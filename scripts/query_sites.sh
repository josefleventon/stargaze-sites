MSG=$(cat <<EOF
{
  "params": {}
}
EOF
)

starsd q wasm contract-state smart $SITES "$MSG"
 
