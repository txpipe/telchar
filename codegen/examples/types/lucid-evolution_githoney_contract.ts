import { Data } from "@lucid-evolution/lucid";

const BoolFalse = Data.Literal("False");

const BoolTrue = Data.Literal("True");

const Bool = Data.Enum([
  BoolFalse,
  BoolTrue,
]);

const ByteArray = Data.Bytes();

const AnyData = Data.Any();

const Int = Data.Integer();

const TypesGithoneyContractRedeemersAddRewards = Data.Literal("AddRewards");

const TypesGithoneyContractRedeemersAssign = Data.Literal("Assign");

const TypesGithoneyContractRedeemersMerge = Data.Literal("Merge");

const TypesGithoneyContractRedeemersClose = Data.Literal("Close");

const TypesGithoneyContractRedeemersClaim = Data.Literal("Claim");

const TypesGithoneyContractRedeemers = Data.Enum([
  TypesGithoneyContractRedeemersAddRewards,
  TypesGithoneyContractRedeemersAssign,
  TypesGithoneyContractRedeemersMerge,
  TypesGithoneyContractRedeemersClose,
  TypesGithoneyContractRedeemersClaim,
]);

const OptionByteArraySome = Data.Tuple([
  ByteArray,
]);

const OptionByteArray = Data.Nullable(OptionByteArraySome);

const TypesSettingsRedeemersUpdateSettings = Data.Literal("UpdateSettings");

const TypesSettingsRedeemersCloseSettings = Data.Literal("CloseSettings");

const AikenTransactionTransactionId = Data.Object({
  hash: ByteArray,
});

const TypesAssetClass = Data.Object({
  policy_id: ByteArray,
  asset_name: ByteArray,
});

const TypesSettingsRedeemers = Data.Enum([
  TypesSettingsRedeemersUpdateSettings,
  TypesSettingsRedeemersCloseSettings,
]);

const TypesTokenInfo = Data.Object({
  asset: TypesAssetClass,
  amount: Int,
});

const TypesWallet = Data.Object({
  payment_key: ByteArray,
  stake_key: OptionByteArray,
});

const ListTypesTokenInfo = Data.Array(TypesTokenInfo);

const OptionTypesWalletSome = Data.Tuple([
  TypesWallet,
]);

const OptionTypesWallet = Data.Nullable(OptionTypesWalletSome);

const RedeemerWrapperTypesGithoneyContractRedeemers = Data.Tuple([
  TypesGithoneyContractRedeemers,
]);

const AikenTransactionDatumNoDatum = Data.Literal("NoDatum");

const AikenTransactionDatumDatumHash = Data.Tuple([
  ByteArray,
]);

const AikenTransactionDatumInlineDatum = Data.Tuple([
  AnyData,
]);

const AikenTransactionDatum = Data.Enum([
  AikenTransactionDatumNoDatum,
  AikenTransactionDatumDatumHash,
  AikenTransactionDatumInlineDatum,
]);

const AikenTransactionOutputReference = Data.Object({
  transaction_id: AikenTransactionTransactionId,
  output_index: Int,
});

const TypesGithoneyDatum = Data.Object({
  admin_wallet: TypesWallet,
  maintainer_wallet: TypesWallet,
  contributor_wallet: OptionTypesWallet,
  bounty_reward_fee: Int,
  deadline: Int,
  merged: Bool,
  initial_value: ListTypesTokenInfo,
});

const TypesSettingsDatum = Data.Object({
  githoney_wallet: TypesWallet,
  bounty_creation_fee: Int,
  bounty_reward_fee: Int,
});
