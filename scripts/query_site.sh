MSG=$(cat <<EOF
{
  "site": {
    "address": "$ADMIN"
  }
}
EOF
)

starsd q wasm contract-state smart $SITES "$MSG"
 
