import { Data } from "@blaze-cardano/sdk";

const AssetName = Data.Bytes();

const Int = Data.Integer();

const PolicyId = Data.Bytes();

const PosixTime = Data.Integer();

const ScriptAddress = Data.Bytes();

const AsteriaTypesAssetClass = Data.Object({
  policy: PolicyId,
  name: AssetName,
});

const AsteriaTypesAsteriaDatum = Data.Object({
  ship_counter: Int,
  shipyard_policy: PolicyId,
});

const AsteriaTypesAsteriaRedeemerAddNewShip = Data.Literal("AddNewShip");

const AsteriaTypesAsteriaRedeemerMine = Data.Literal("Mine");

const AsteriaTypesAsteriaRedeemerConsumeAsteria = Data.Literal("ConsumeAsteria");

const AsteriaTypesAsteriaRedeemer = Data.Enum([
  AsteriaTypesAsteriaRedeemerAddNewShip,
  AsteriaTypesAsteriaRedeemerMine,
  AsteriaTypesAsteriaRedeemerConsumeAsteria,
]);

const AsteriaTypesFuelRedeemerMintFuel = Data.Literal("MintFuel");

const AsteriaTypesFuelRedeemerBurnFuel = Data.Literal("BurnFuel");

const AsteriaTypesFuelRedeemer = Data.Enum([
  AsteriaTypesFuelRedeemerMintFuel,
  AsteriaTypesFuelRedeemerBurnFuel,
]);

const AsteriaTypesPelletDatum = Data.Object({
  pos_x: Int,
  pos_y: Int,
  shipyard_policy: PolicyId,
});

const AsteriaTypesPelletRedeemerProvide = Data.Object({
  amount: Int,
});

const AsteriaTypesPelletRedeemer = Data.Nullable(AsteriaTypesPelletRedeemerProvide);

const AsteriaTypesShipDatum = Data.Object({
  pos_x: Int,
  pos_y: Int,
  ship_token_name: AssetName,
  pilot_token_name: AssetName,
  last_move_latest_time: PosixTime,
});

const AsteriaTypesShipRedeemerMoveShip = Data.Object({
  delta_x: Int,
  delta_y: Int,
});

const AsteriaTypesShipRedeemerGatherFuel = Data.Object({
  amount: Int,
});

const AsteriaTypesShipRedeemerMineAsteria = Data.Literal("MineAsteria");

const AsteriaTypesShipRedeemerQuit = Data.Literal("Quit");

const AsteriaTypesShipRedeemer = Data.Enum([
  AsteriaTypesShipRedeemerMoveShip,
  AsteriaTypesShipRedeemerGatherFuel,
  AsteriaTypesShipRedeemerMineAsteria,
  AsteriaTypesShipRedeemerQuit,
]);

const AsteriaTypesShipyardRedeemerMintShip = Data.Literal("MintShip");

const AsteriaTypesShipyardRedeemerBurnShip = Data.Literal("BurnShip");

const AsteriaTypesShipyardRedeemer = Data.Enum([
  AsteriaTypesShipyardRedeemerMintShip,
  AsteriaTypesShipyardRedeemerBurnShip,
]);

const AsteriaTypesSpeed = Data.Object({
  distance: Int,
  time: Int,
});

    
