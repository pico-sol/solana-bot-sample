各プログラムはbinフォルダの中に入っているので、以下のような形でそれぞれ実行してください。

1. write_keypair_fileはbin/write_keypair_file/src/main.rsの中のyour secret keyをbase58 String型の秘密鍵に置き換えて実行してください。
2. read_keypair_file, token_list_test, jup_swap_api_testはlib/util/const_str.rsのPAYERを自身のキーペアファイルのpathに置き換えて実行してください。
3. jup_swap_api_testはトランザクション発行部分のコードをコメントアウトしてあります。コメントアウトを外して実行してください。
   0.01 SOLをUSDCにスワップするトランザクションが発行されます。
   ウォレット内にあらかじめガス代＋0.01 SOL以上のSOLが必要です。

【注意】：秘密鍵の管理は厳重に行ってください。流出した場合ウォレット内の資産がすべてコントロールされることになります。
　　　　　jup_swap_api_testはメインネットで実際にトランザクションが発行されますが、このコードを使用したいかなる損失も補償できません。
```
git clone https://github.com/pico-sol/solana-bot-sample.git
cd solana-bot-sample

carog r -r --bin write_keypair_file

carog r -r --bin read_keypair_file

carog r -r --bin token_list_test

carog r -r --bin pyth_test

carog r -r --bin jup_swap_api_test
```
