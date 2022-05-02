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
