* pub,pub(crate)の違い -> crate単位でglobal(mod でimportされる間は使える).一方完全に別moduleからのimportはできない

## TODO:
* ~~storageへの書き込みは &[u8] にしたいけど、ちょっとlifetimeの引き回しがむずそう~~
* マルチスレッド環境の考慮


## 日報
5/2
* やったこと
  * fsのデータ構造を見直して
    * ファイル名とmetadataのmapを持つように
    * fsは空いてるstorage領域を把握しておかないといけないので、空いてる領域をStackで管理するようにした.
* 次回
  * Rustでstackの実装 
  * Fsを実際に動作させる (stackができれば動きそう)
  * testを少し追加する.
  * get(read) にBorrowをいれる


5/3
* やったこと
  * stackの実装,test
  * それをfsに組み込む途中
  * 所有権あたりで詰まっているので調査...
* 次回
  * 所有権で、なんで弾かれてるのかよく理解できていないので、この辺をもう一回復讐する(なんやかんやできちんと勉強してなかったな....)
    * https://zenn.dev/j5ik2o/articles/918c54411d5a61
