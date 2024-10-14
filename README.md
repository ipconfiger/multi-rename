# multi-rename
Multi rename files

多文件批量重命名

在老NAS的服务器上找到了很多以前BT下载的美剧，但是在看的时候，文件名过长和复杂
比如这样
![WeChat6061fbf543f18236836680e342070923](https://github.com/user-attachments/assets/eb9f9056-5277-4a84-b685-fc385792ecac)

在播放的时候就会很蛋痛，需要批量的截断文件名，得到更容易读的名字，也更容易显示
比如文件：星际迷航：奇异新世界.第一季.2022.EP01.HD1080P.X264.AAC.English.CHS-ENG.BDYS.mp4
拆开后 星际迷航 奇异新世界 第一季 2022 EP01 HD1080P X264 AAC English CHS-ENG BDYS .mp4 其中 其实只需要 EP01.mp4 也可以 星际迷航_奇异新世界_EP01.mp4
执行后：

![WeChat95d814c662b9942744e939ac2df3db05](https://github.com/user-attachments/assets/9188f63b-36b4-4e91-8a71-276df418aec9)

把文件名爆开编号后，我们发现，0和3是我们需要的，就在输入 mrn --try --select 0,3

![WeChat09cfa157f112cf0dd8903e1117280dee](https://github.com/user-attachments/assets/c4c54379-78d2-46e9-99c0-c5cf27ebe720)

能看到得到的新的文件名就是

![WeChatc50f598b71f1e4c162e9e8c390756e95](https://github.com/user-attachments/assets/4ce27974-bce3-4305-86bc-584422b14dea)

检查没问题，就加个参数 --do，即是：mrn --try --select 0,3 --do 得到结果：

![WeChatcdef9907d74850dfb404a36db653d963](https://github.com/user-attachments/assets/68d23a2d-5806-4928-b6e2-07caeef03910)

最后效果如下：

![WeChat30f383ac239a93624777120d85613ed5](https://github.com/user-attachments/assets/98a846f5-761f-4102-8bf8-43f0d421bdb8)
