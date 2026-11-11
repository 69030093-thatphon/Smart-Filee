# Smart File Manager Pro (Smart-Filee) v4.3.1 — Rust Edition

ชุดเครื่องมือเดสก์ท็อปจัดการไฟล์ประสิทธิภาพสูงแบบ local-first พัฒนาใหม่ด้วยภาษา **Rust** เต็มรูปแบบสำหรับ Windows 10/11 (และรองรับ macOS / Linux อย่างสมบูรณ์)
- **สแตนด์อโลนไฟล์เดียว (<9 MB):** ไม่ต้องพึ่งพา Python runtime, ไม่ต้องมี `.venv`, ไม่ต้องมีไดรเวอร์ภายนอก
- **เปิดแอปทันที (<100ms):** ลดการใช้หน่วยความจำลง 79.5% (เหลือ ~18 MB RSS) และทำงานเร็วกว่าเดิมสูงสุด 6.6 เท่า
- **UI ทันสมัยด้วย `egui`:** ธีม Premium Light Violet, รองรับ 6 ชุดสี, รองรับภาษาไทยและอังกฤษ
- **Local-first & ปลอดภัย:** บันทึก SQLite WAL mode (`sfm.db`), ระบบ Undo ปลอดภัย 100%, ป้องกันโฟลเดอร์ระบบ
- **Supabase Cloud Identity:** ล็อกอินและยืนยันตัวตนผ่าน Supabase Auth พร้อม loopback OTP บนพอร์ต 8765

---

## วิธีเริ่มใช้งาน (Rust Edition)

### 1. รันแอปพลิเคชัน
```bash
cargo run -p smart_filee_app
```
หรือบน Windows ดับเบิลคลิก `run.bat`

### 2. บิลด์ Release Executable ขนาดกะทัดรัด
```bash
cargo build --release -p smart_filee_app
```
ไฟล์ไบนารีจะอยู่ที่ `target/release/smart_filee` (หรือ `smart_filee.exe` บน Windows ขนาดเพียง 8.7 MB)

### 3. รันการทดสอบและ Selftest
```bash
cargo test --workspace                       # ทดสอบ 53 unit & benchmark tests
cargo run -p smart_filee_app -- --selftest   # ทดสอบ headless validation อัตโนมัติ
```

### 4. สร้างตัวติดตั้ง Windows Installer (.exe)
รัน Inno Setup ผ่านไฟล์ `installer.iss` หรือดับเบิลคลิก `build_exe.bat` เพื่อสร้างตัวติดตั้ง `SmartFileManagerPro-Setup-v4.3.1.exe`

---

## มีอะไรใหม่ใน 4.3.1 (Rust Edition)
- **เขียนใหม่ด้วย Rust 100%:** แยกโครงสร้างเป็น 17 crates ภายใต้ Cargo Workspace ปราศจาก Python dependencies
- **ความเร็วสูงขึ้น 6.6 เท่า:** ค้นหาและท่องโครงสร้างโฟลเดอร์ได้ 232,500 ไฟล์/วินาที
- **SQLite WAL Batch Writing เร็วขึ้น 6.2 เท่า:** บันทึกประวัติ undo 5,000 ไฟล์ในเวลาเพียง 0.013 วินาที
- **UI Desktop ใหม่ด้วย `egui` / `eframe`:** เมนูสลับหน้า 14 เครื่องมือ, แถบไอคอนบางเฉียบ, ค้นหาเร็วด้วย `Ctrl+K`
- **ระบบความปลอดภัยแบบสมบูรณ์:** ป้องกันไดเรกทอรีระบบ, ย้ายไฟล์แบบ atomic พร้อม rollback journal, ผสานถังรีไซเคิลระบบผ่าน `trash`

---

## มีอะไรใหม่ใน 4.3 (Legacy Python)
- **ที่เก็บข้อมูลแบบ SQLite** (`db.py`, ไฟล์ `~/.smart_file_manager_pro/sfm.db`, ใช้ `sqlite3` ของไลบรารีมาตรฐาน ไม่ต้องติดตั้งเพิ่ม) มาแทนที่ไฟล์ JSON ของ undo, ล็อกข้อความกิจกรรม และกฎที่เคยเก็บใน QSettings:
  - `undo_batches` + `undo_moves` (ความสัมพันธ์ 1 ต่อหลาย, `ON DELETE CASCADE`): ประวัติ undo คงอยู่หลังรีสตาร์ต และรายการที่ undo แล้วจะถูกเก็บเป็นประวัติ (`status`)
  - `activity_log`: ค้นหาได้ (มีช่องกรองใหม่ในหน้า Activity Log) และตัดรายการเก่าอัตโนมัติ
  - `rules`: กฎจัดระเบียบแบบกำหนดเองที่เรียงลำดับได้
  - `hash_cache`: Duplicate Finder นำค่า SHA-256 กลับมาใช้ เมื่อขนาด + เวลาแก้ไขของไฟล์ไม่เปลี่ยน ทำให้สแกนซ้ำเร็วขึ้นมาก
  - `scan_history`: บันทึกการสแกนไฟล์ซ้ำทุกครั้ง (จำนวนไฟล์, กลุ่ม, ไบต์ที่สูญเปล่า, cache hit/miss, เวลาที่ใช้)
- ไฟล์เดิม `undo_journal.json`, `activity.log` และกฎใน QSettings จะถูกนำเข้าครั้งเดียวเมื่อเริ่มใช้งานครั้งแรก และไม่ถูกแตะต้องบนดิสก์
- ธีม ภาษา ฟอนต์ ตำแหน่งหน้าต่าง และพาธ ยังเก็บใน `QSettings` เหมือนเดิม

## มีอะไรใหม่ใน 4.2
- **ออกแบบ UI ใหม่** (ตรงกับ mockup โทนม่วงอ่อนใหม่): แถบด้านข้างแบบไอคอนบางๆ พร้อมโลโก้ SF, เครื่องมือหลักเจ็ดอย่าง, ปุ่ม **⋯** สำหรับหน้าอื่น (Similar Photos, File Search, Processes, Network, System, Activity Log) และ Settings ที่ด้านล่าง ไม่มีแถบบน/ล่าง
- **Dashboard**: คำทักทาย, วงแหวนคะแนน *ความเป็นระเบียบของไฟล์* (0-100 ดู `core.tidy_score`) และคำแนะนำสูงสุดสามข้อที่คำนวณจากการอ่านโฟลเดอร์จัดระเบียบของคุณแบบเร็วและอ่านอย่างเดียว พร้อมการ์ดพื้นที่ว่าง / CPU / จำนวน undo ช่องค้นหาจะเปิด Ctrl+K (ค้นหาไฟล์ได้ด้วย)
- **File Organizer** แสดงตัวอย่างเอง (ไม่มีปุ่ม Scan): มีชิปเลือกโหมด, แถบโฟลเดอร์ที่จะสร้าง, รายการไฟล์ที่ติ๊กไว้, ช่อง auto-organize และปุ่ม `Organize N files`
- **Disk Analyzer, Duplicate Finder, Cleanup, Bulk Renamer, Settings** สร้างใหม่เป็นเลย์เอาต์แบบการ์ด ตัวเลือกสแกนที่ดีไซน์ไม่มีที่ว่าง (ขนาดขั้นต่ำของไฟล์ซ้ำ; โฟลเดอร์/ขนาด/อายุของ cleanup) อยู่ใต้ลิงก์เล็กๆ **Options…** และไฟล์ใหญ่สุด 100 อันดับเปิดได้จากลิงก์ท้ายหน้า
- Settings กลับมามีชิปธีม ฟอนต์และขนาด ภาษา และอัตรารีเฟรชอีกครั้ง

## มีอะไรใหม่ใน 4.0
- หน้า **Cleanup**: ไฟล์ชั่วคราว, ไฟล์ใหญ่ที่ไม่ได้ใช้ N วัน, โฟลเดอร์ว่าง → ส่งไปถังรีไซเคิล ไฟล์ใหญ่ที่ไม่ได้ใช้จะไม่ถูกติ๊กเลือกไว้ล่วงหน้า
- หน้า **Similar Photos**: จับคู่แบบ perceptual (hash 3 แบบ) สำหรับภาพถ่ายต่อเนื่อง / ภาพที่ย่อขนาด / ภาพที่บันทึกซ้ำ พร้อมภาพย่อ เก็บสำเนาที่ใหญ่สุดไว้ ย้ายไป `_Similar` (undo ได้) หรือส่งไปถังรีไซเคิล
- โหมดจัดระเบียบ **รูปภาพตามวันที่ถ่าย** (วันที่ EXIF หากไม่มีใช้วันที่ไฟล์)
- **รายงานข้อผิดพลาด**: ข้อผิดพลาดที่ไม่คาดคิดจะถูกเขียนลง `~/.smart_file_manager_pro/crash.log` และมีกล่องข้อความบอกตำแหน่งไฟล์
- **CI** (`.github/workflows/ci.yml`): รันเทสต์บน Linux + **Windows**, ตรวจสอบการเริ่มต้นด้วย `--selftest` และสร้าง `.exe` บน Windows พร้อม smoke test แล้วอัปโหลดเป็น artifact
- แถบด้านข้างเลื่อนได้เมื่อหน้าต่างเตี้ยหรือฟอนต์ใหญ่

## มีอะไรใหม่ใน 3.2
- **ปรับแต่ง UI**: ไอคอนเมนูเรียงเป็นคอลัมน์ตรงกัน, ไม่มีลูกศรเรียงลำดับค้างจนกว่าจะคลิกหัวคอลัมน์, ปุ่มใน Settings เป็นระเบียบขึ้น, ภาษาไทยใช้ Leelawadee UI ขนาด 11 pt เป็นค่าเริ่มต้น, หน่วย uptime เป็นภาษาไทย

## มีอะไรใหม่ใน 3.1
- **ตั้งค่าฟอนต์** (Settings → Font): เลือกฟอนต์ที่ติดตั้งในเครื่องและขนาด (8–20 pt) หัวข้อ ตัวเลขใหญ่ และแถบด้านข้างจะปรับตาม ชุดฟอนต์เริ่มต้นมีตัวสำรองที่รองรับภาษาไทย (Leelawadee UI, Tahoma, Noto Sans Thai)

## มีอะไรใหม่ใน 3.0
- **ความปลอดภัย**: ปฏิเสธโฟลเดอร์ระบบและรากของไดรฟ์สำหรับการย้าย/เปลี่ยนชื่อ/สำรองข้อมูล, การสแกนรายงานจำนวนไฟล์ที่อ่านไม่ได้ซึ่งถูกข้าม, **ประวัติ Undo ถูกบันทึกลงดิสก์** และคงอยู่หลังรีสตาร์ต
- **การใช้งาน**: คลิกหัวคอลัมน์ใดก็ได้เพื่อเรียงลำดับ (ขนาด/วันที่เรียงตามตัวเลข), คลิกขวาที่แถว → *Open file / Show in folder / Copy path* (โปรแกรมและสคริปต์จะไม่ถูกรันจากที่นี่)
- **กฎกำหนดเอง** ใน Organizer (`*.psd` → `Design`, `invoice` → `Money`, …) บันทึกไว้ใช้ข้ามเซสชัน
- ตัวเลือก **ถังรีไซเคิล** สำหรับไฟล์ซ้ำ (กู้คืนจากถังได้ เป็นการกระทำเดียวที่ undo ในแอปไม่ได้)
- **จัดระเบียบอัตโนมัติ (watch folder)** พร้อมไอคอนใน system tray: ไฟล์ใหม่ที่ไม่เปลี่ยนแปลงนาน 5 วินาทีจะถูกย้ายตามโหมดและกฎของคุณ ทุกชุดการย้าย undo ได้
- **หน้าจอภาษาไทย**

## ฟีเจอร์
| หน้า | ทำอะไรได้ |
|---|---|
| Dashboard | กราฟ sparkline CPU / RAM / เครือข่ายแบบสด, เกจไดรฟ์ระบบ, ภาพรวม, ทางลัด, กิจกรรมล่าสุด |
| File Organizer | จัดตามประเภท, ประเภท + เดือน หรือเดือน; กฎกำหนดเอง; ดูตัวอย่าง; เอาติ๊กไฟล์ออกได้; จัดระเบียบอัตโนมัติ |
| Disk Analyzer | ไฟล์ใหญ่สุด 100 อันดับ, แผนภูมิพื้นที่ตามประเภทและโฟลเดอร์ที่ใหญ่สุด, ส่งออก CSV |
| Duplicate Finder | ขนาด → partial hash → SHA-256; ย้ายไป `_Duplicates` (เก็บไฟล์เก่าสุดไว้) หรือถังรีไซเคิล |
| Similar Photos | จับคู่แบบ perceptual พร้อมภาพย่อ; ย้ายไป `_Similar` หรือถังรีไซเคิล |
| Cleanup | ไฟล์ชั่วคราว, ไฟล์ใหญ่ที่ไม่ได้ใช้, โฟลเดอร์ว่าง → ถังรีไซเคิล |
| File Search | ค้นด้วยชื่อหรือ wildcard (`*.pdf`), ขนาดขั้นต่ำ, ส่งออก CSV |
| Bulk Renamer | ค้นหา/แทนที่, คำนำหน้า/ต่อท้าย, ตัวพิมพ์, ช่องว่าง, ใส่เลขลำดับ; ดูตัวอย่างสด; ตรวจจับชื่อชนกัน |
| Backup | คัดลอกทางเดียวเฉพาะไฟล์ใหม่/ที่เปลี่ยน พร้อมตัวอย่าง; ไม่ลบอะไรเลย |
| Processes | ดูโปรเซสอันดับต้นๆ (อ่านอย่างเดียว) |
| Network Tools | อินเทอร์เฟซ, ping + DNS, ตรวจพอร์ต TCP ทีละพอร์ต |
| System Info / Activity Log / Settings | ภาษา, อัตรารีเฟรช |

ทางลัด: **Ctrl+K** ตัวสลับด่วน, **Ctrl+1…9** สลับหน้า ลากโฟลเดอร์มาวางที่ช่องพาธใดก็ได้

## วิธีรัน
```powershell
.\run.bat
```
หรือ:
```powershell
py -m venv .venv
.\.venv\Scripts\Activate.ps1
pip install -r requirements.txt
python app.py
```

## สร้างไฟล์ .exe แบบสแตนด์อโลน
```powershell
.\build_exe.bat
```
ผลลัพธ์: `dist\SmartFileManagerPro.exe` (ไม่มีลายเซ็น Windows SmartScreen จึงอาจแจ้งเตือน)

## การทดสอบ
```bash
python -m unittest discover -s tests -v     # core + DB + UI แบบ headless + ความครบถ้วนของ i18n (58 เทสต์)
```

## โครงสร้างไฟล์
```
app.py       UI                    core.py      ลอจิกไฟล์/เครือข่ายทั้งหมด (ไม่พึ่ง Qt, มี unit test)
widgets.py   แผนภูมิ, toast, item  themes.py    ธีมสีและ stylesheet
crash.py     ตัวจัดการรายงานข้อผิดพลาด  .github/  workflow ของ CI (Linux + Windows, สร้าง exe)
i18n.py / i18n_th.py  การแปลภาษา    tests/       เทสต์ของ core, UI และ i18n
```

## ความปลอดภัย
- ไม่มีอะไรถูกย้ายจนกว่าคุณจะยืนยัน การลบเพียงอย่างเดียวคือปุ่มเสริม *Recycle checked…* (ส่งไปถังรีไซเคิลของระบบปฏิบัติการ)
- ไม่เขียนทับไฟล์เดิม (ใช้ชื่อ `file (1).pdf` เมื่อชื่อชนกัน)
- ข้ามไฟล์ดาวน์โหลดที่ยังไม่เสร็จ ไฟล์ซ่อน และ symlink
- Undo ครอบคลุมการย้าย/เปลี่ยนชื่อ 20 ชุดล่าสุด รวมถึงหลังรีสตาร์ต (รายการที่ไฟล์หายไปแล้วจะถูกตัดออก)
- Auto-organize ปิดไว้เป็นค่าเริ่มต้น และจะขอการยืนยันเมื่อเปิดใช้

## ข้อจำกัดที่ทราบ
- พัฒนาและทดสอบแบบ headless บน Linux ส่วน GitHub Actions workflow ที่แนบมาจะรันเทสต์ชุดเดียวกันและตรวจการเริ่มต้นบน Windows — push ขึ้น GitHub เพื่อให้ได้การยืนยันนั้น
- ยังไม่รองรับ: พาธ Windows ที่ยาวเกิน 260 ตัวอักษร และไฟล์ที่ถูกโปรแกรมอื่นล็อก (จะรายงานเป็นข้อผิดพลาด ไม่บังคับดำเนินการ)
- บรรทัดในล็อกกิจกรรมและผลลัพธ์ ping จากระบบปฏิบัติการยังเป็นภาษาอังกฤษ
- Undo ไม่ครอบคลุมการส่งไปถังรีไซเคิลและการคัดลอกของ Backup


## Online membership (PostgreSQL)

This release includes an optional-to-deploy FastAPI membership backend with PostgreSQL storage. The desktop app connects to it over HTTPS; PostgreSQL credentials and token-signing secrets stay on the server. The existing SQLite database remains local for file-operation metadata. There is one standard member tier plus an Admin role (no Pro membership or paid feature gates). See [MEMBERSHIP_SETUP.md](MEMBERSHIP_SETUP.md) for deployment, configuration, and first-admin setup.


## Online membership (Supabase)

This build uses Supabase Auth for email/password registration and sign-in. Copy `.env.example` to `.env` if needed; the included `.env` is configured for the connected project using its public publishable key. Never replace it with a `service_role` or secret key.

- `Account → Profile & devices…` manages the display name and shows registered device records.
- `Account → Sign out and exit` ends the current Supabase session.
- The existing SQLite database remains local. File paths, file contents, and local activity history are not uploaded.
- Standard user accounts only; no Pro Member tier.
- See `MEMBERSHIP_SETUP.md` for setup and security notes.
