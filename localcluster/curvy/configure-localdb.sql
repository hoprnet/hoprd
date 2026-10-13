-- Configure only the launcher-owned, fresh local database, before workers start.
-- Schemas and circuit metadata come from the published image; no migrations run.
BEGIN;
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM indexer.notes) OR
     EXISTS (SELECT 1 FROM relayer.relay_submissions) OR
     EXISTS (SELECT 1 FROM batch_prover.batch_runs) THEN
    RAISE EXCEPTION 'refusing to configure a database containing prior activity';
  END IF;
END $$;
CREATE TEMP TABLE local_circuits AS
  SELECT * FROM reference.circuit_configs
  WHERE network_id = (SELECT min(network_id) FROM reference.circuit_configs);
TRUNCATE reference.bridge_mappings, reference.networks_currencies,
  reference.circuit_configs, reference.networks, reference.currencies CASCADE;
INSERT INTO reference.networks
  (id,name,network_group,testnet,slip0044,flavour,chain_id,blockexplorer_url,
   multi_call_contract_address,vault_contract_address,aggregator_contract_address,
   portal_factory_contract_address,vault_contract_version,default_aggregator,portal_shield_enabled)
VALUES (1,'HOPR Localnet','Ethereum',true,60,'evm','31337','http://localhost',
  :'multicall', :'vault',
  :'aggregator', :'factory',
  '1.0',true,false);
INSERT INTO reference.currencies (id,name,symbol,decimals,price)
  VALUES (1,'Local native currency','ETH',18,1),(2,'Wrapped HOPR','wxHOPR',18,1);
INSERT INTO reference.networks_currencies
  (network_id,currency_id,contract_address,native_currency,vault_token_id)
VALUES (1,1,'0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee',true,1),
       (1,2,:'token',false,3);
INSERT INTO reference.circuit_configs
  (network_id,type,wasm_key,witness_engine,witness_graph_key,witness_graph_sha256,
   zkey_key,zkey_sha256,tree_depth,max_inputs,max_outputs,batch_size,group_fee,version)
SELECT 1,type,wasm_key,witness_engine,witness_graph_key,witness_graph_sha256,
   zkey_key,zkey_sha256,tree_depth,max_inputs,max_outputs,batch_size,group_fee,version
FROM local_circuits;
INSERT INTO reference.circuit_configs
  (network_id,type,witness_engine,witness_graph_key,witness_graph_sha256,
   zkey_key,zkey_sha256,tree_depth,max_inputs,max_outputs,batch_size,group_fee)
VALUES (1,'pending_notes_commitment','curvy-graph-v1',:'graph',
  :'graph_sha256',:'zkey',:'zkey_sha256',30,0,0,5,0)
ON CONFLICT (network_id,type) DO UPDATE SET
  witness_engine=excluded.witness_engine,witness_graph_key=excluded.witness_graph_key,
  witness_graph_sha256=excluded.witness_graph_sha256,zkey_key=excluded.zkey_key,
  zkey_sha256=excluded.zkey_sha256,tree_depth=30,max_inputs=0,max_outputs=0,batch_size=5,group_fee=0;
SELECT setval(pg_get_serial_sequence('reference.networks','id'),1);
SELECT setval(pg_get_serial_sequence('reference.currencies','id'),2);
COMMIT;
