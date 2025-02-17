import { Data } from "@blaze-cardano/sdk";

const ByteArray = Data.Bytes();

const Int = Data.Integer();

const AikenTransactionCredentialCredentialScriptCredential = Data.Tuple([
  ByteArray,
]);

const AikenTransactionCredentialCredentialVerificationKeyCredential = Data.Tuple([
  ByteArray,
]);

const AikenTransactionCredentialCredential = Data.Enum([
  AikenTransactionCredentialCredentialVerificationKeyCredential,
  AikenTransactionCredentialCredentialScriptCredential,
]);

const AikenTransactionCredentialReferencedAikenTransactionCredentialCredentialInline = Data.Tuple([
  AikenTransactionCredentialCredential,
]);

const AikenTransactionCredentialReferencedAikenTransactionCredentialCredentialPointer = Data.Object({
  slot_number: Int,
  transaction_index: Int,
  certificate_index: Int,
});

const AikenTransactionCredentialReferencedAikenTransactionCredentialCredential = Data.Enum([
  AikenTransactionCredentialReferencedAikenTransactionCredentialCredentialInline,
  AikenTransactionCredentialReferencedAikenTransactionCredentialCredentialPointer,
]);

const OptionAikenTransactionCredentialReferencedAikenTransactionCredentialCredentialSome = Data.Tuple([
  AikenTransactionCredentialReferencedAikenTransactionCredentialCredential,
]);

const OptionAikenTransactionCredentialReferencedAikenTransactionCredentialCredential = Data.Nullable(OptionAikenTransactionCredentialReferencedAikenTransactionCredentialCredentialSome);

const AikenTransactionCredentialAddress = Data.Object({
  payment_credential: AikenTransactionCredentialCredential,
  stake_credential: OptionAikenTransactionCredentialReferencedAikenTransactionCredentialCredential,
});

const JpgTypesPayout = Data.Object({
  address: AikenTransactionCredentialAddress,
  amount_lovelace: Int,
});

const ListJpgTypesPayout = Data.Array(JpgTypesPayout);

const AskDatum = Data.Object({
  payouts: ListJpgTypesPayout,
  owner: ByteArray,
});

const AskRedeemerBuy = Data.Object({
  payout_outputs_offset: Int,
});

const AskRedeemer = Data.Nullable(AskRedeemerBuy);
