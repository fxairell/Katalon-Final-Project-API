Disclaimer
Perhatian file ini berisi ringkasan dari file project Katalon pada Folder Final Project1_F.X.
Airell V.S.W._KSAT006ONL013 yang terdapat pada Folder Sesi 11 bagian dari Automation Testing with
Katalon Studio. Adapun ringkasan merupakan deskripsi singkat mengenai hal yang tercantum pada file
excel ScenarioTestDocs-20220706-F.X. Airell Valerio Satrio W (API).xlsx.

Nama			: F.X. Airell Valerio Satrio Wibowo
Kode Peserta	: KSAT006ONL013
Link Github		: https://github.com/fxairell/BTDP-Automation-Katalon
Ringkasan		:

Sheet yang terdapat dalam file:
- User
- Test Case Branch
- Test Case Flow

Ringkasan:
a) User
	User merupakan sheet yang digunakan untuk melihat berbagai Test Case dengan Test Scenarionya,
dilengkapi dengan Step, Test Data, dan nilai Expected-Actual untuk penentuan Passed, Failed, atau Not
Executed. Functional Test digunakan sebagai pemecah alur Test Case dan membaginya menjadi beberapa
kelompok.
b) Test Case Branch
	Test Case Branch merupakan sheet yang digunakan untuk menjabarkan percabangan skenario yang
digunakan dalam berbagai Test Case. Branch diberi nomor untuk memudahkan menentukan alur dari Test Case.
Total terdapat 16 cabang yang digunakan dalam berbagai Test Case. Test Scenario Branch digunakan untuk
menjabarkan Functional Test dari test web sederhana. Test Case ID dibentuk dari perpaduan singkatan
Test Scenario Branch dan penomoran urut dari alur yang digunakan.
c) Test Case Flow
	Test Case Flow merupakan sheet yang digunakan untuk mendefinisikan alur dari setiap Test Case.
Total terdapat 48 Test Case berbeda beserta alur dan penjabaran Positif atau Negatifnya. Alur juga
dilengkapi dengan Pre Condition untuk mempersingkat penulisan alur Test Case.

Beberapa catatan tambahan:
a) [SOLVED] Data pada website memiliki sifat yang cukup dinamis sehingga kurang pas apabila menggunakan metode
   testing Data Driven (DDT) yang bersifat statis seperti Excel. Test Case sengaja digagalkan karena
   metode yang digunakan kurang cocok dalam testing API dengan model seperti ini. Pengembangan metode
   diperlukan agar dapat merekam penampilan seluruh ID dan mendapatkan detail dari booking yang
   dilakukan.

   [Answer] Penambahan pernyataan kondisional cukup membantu dalam mengurangi ketidakpastian data.