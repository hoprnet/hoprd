-- Read-only compatibility check. Schema and seeds belong to the Curvy release
-- image; hoprd must not carry a second copy of the upstream migrations.
WITH localnet AS (
  SELECT * FROM reference.networks WHERE chain_id = '31337'
)
SELECT jsonb_build_object(
  'postgres_version', current_setting('server_version'),
  'networks', (SELECT count(*) FROM localnet),
  'aggregator', (SELECT aggregator_contract_address FROM localnet),
  'vault', (SELECT vault_contract_address FROM localnet),
  'token_present', EXISTS (
    SELECT 1 FROM reference.networks_currencies nc
    JOIN localnet n ON n.id = nc.network_id
    JOIN reference.currencies c ON c.id = nc.currency_id
    WHERE nc.vault_token_id = 3 AND coalesce(nc.decimal_override, c.decimals) = 18
  ),
  'schemas_present',
    to_regclass('indexer.notes') IS NOT NULL AND
    to_regclass('relayer.relay_submissions') IS NOT NULL AND
    to_regclass('batch_prover.batch_runs') IS NOT NULL,
  'pending', (
    SELECT to_jsonb(c) FROM reference.circuit_configs c
    JOIN localnet n ON n.id = c.network_id
    WHERE c.type = 'pending_notes_commitment'
  )
);
