import { ByteString, Dict } from "@meshsdk/core";

type AssetName = ByteString;

type Int = number;

type PolicyId = ByteString;

type PosixTime = number;

type ScriptAddress = ByteString;

type AsteriaTypesAssetClass = Dict<any> & {
    policy: PolicyId,
    name: AssetName,
};

type AsteriaTypesAsteriaDatum = Dict<any> & {
    ship_counter: Int,
    shipyard_policy: PolicyId,
};

type AsteriaTypesAsteriaRedeemerAddNewShip = "AddNewShip";