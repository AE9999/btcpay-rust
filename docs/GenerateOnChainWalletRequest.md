# GenerateOnChainWalletRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**existing_mnemonic** | Option<**String**> | An existing BIP39 mnemonic seed to generate the wallet with | [optional]
**passphrase** | Option<**String**> | A passphrase for the BIP39 mnemonic seed | [optional]
**account_number** | Option<**f32**> | The account to derive from the BIP39 mnemonic seed | [optional][default to 0]
**save_private_keys** | Option<**bool**> | Whether to store the seed inside BTCPay Server to enable some additional services. IF `false` AND `existingMnemonic` IS NOT SPECIFIED, BE SURE TO SECURELY STORE THE SEED IN THE RESPONSE! | [optional][default to false]
**import_keys_to_rpc** | Option<**bool**> | Whether to import all addresses generated via BTCPay Server into the underlying node wallet. (Private keys will also be imported if `savePrivateKeys` is set to true. | [optional][default to false]
**word_list** | Option<**String**> | If `existingMnemonic` is not set, a mnemonic is generated using the specified wordList. | [optional][default to WordList_English]
**word_count** | Option<**f32**> | If `existingMnemonic` is not set, a mnemonic is generated using the specified wordCount. | [optional][default to WordCount__12]
**script_pub_key_type** | Option<**String**> | the type of wallet to generate | [optional][default to ScriptPubKeyType_Segwit]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


