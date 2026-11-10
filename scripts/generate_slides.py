"""
Generate Smart File Manager Pro Presentation Slides in PPTX format.
Aligns strictly with the 10 evaluation topics and presentation criteria.
"""

from pptx import Presentation
from pptx.util import Inches, Pt
from pptx.dml.color import RGBColor
from pptx.enum.text import PP_ALIGN
from pptx.enum.shapes import MSO_SHAPE

def create_presentation():
    prs = Presentation()
    prs.slide_width = Inches(13.333)
    prs.slide_height = Inches(7.5)
    blank_slide_layout = prs.slide_layouts[6]

    # Colors
    BG_DARK = RGBColor(22, 27, 46)        # Deep Navy
    CARD_BG = RGBColor(32, 39, 66)        # Slate Card
    CARD_BORDER = RGBColor(75, 85, 125)   # Border
    ACCENT_PURPLE = RGBColor(129, 90, 243) # Primary Purple
    ACCENT_GREEN = RGBColor(0, 184, 148)  # Success Green
    ACCENT_CYAN = RGBColor(9, 132, 227)   # Tech Cyan
    ACCENT_AMBER = RGBColor(253, 203, 110)# Highlight Gold
    TEXT_WHITE = RGBColor(245, 246, 250)  # Main Text
    TEXT_MUTED = RGBColor(178, 190, 195)  # Secondary Text

    def set_slide_background(slide):
        background = slide.background
        fill = background.fill
        fill.solid()
        fill.fore_color.rgb = BG_DARK

    def add_header(slide, title_text, category_text, presenter_name):
        set_slide_background(slide)

        # Category / Section Pill
        cat_box = slide.shapes.add_textbox(Inches(0.8), Inches(0.4), Inches(8), Inches(0.4))
        tf_cat = cat_box.text_frame
        tf_cat.word_wrap = True
        p_cat = tf_cat.paragraphs[0]
        p_cat.text = category_text.upper()
        p_cat.font.size = Pt(11)
        p_cat.font.bold = True
        p_cat.font.color.rgb = ACCENT_PURPLE

        # Main Title
        title_box = slide.shapes.add_textbox(Inches(0.8), Inches(0.75), Inches(8.5), Inches(0.8))
        tf_title = title_box.text_frame
        tf_title.word_wrap = True
        p_title = tf_title.paragraphs[0]
        p_title.text = title_text
        p_title.font.size = Pt(22)
        p_title.font.bold = True
        p_title.font.color.rgb = TEXT_WHITE

        # Presenter Badge (Top Right - Required by rubric)
        pres_shape = slide.shapes.add_shape(MSO_SHAPE.ROUNDED_RECTANGLE, Inches(9.3), Inches(0.5), Inches(3.2), Inches(0.55))
        pres_shape.fill.solid()
        pres_shape.fill.fore_color.rgb = RGBColor(40, 48, 80)
        pres_shape.line.color.rgb = ACCENT_PURPLE
        pres_shape.line.width = Pt(1.5)
        tf_pres = pres_shape.text_frame
        p_pres = tf_pres.paragraphs[0]
        p_pres.text = f"👤 ผู้บรรยาย: {presenter_name}"
        p_pres.font.size = Pt(11)
        p_pres.font.bold = True
        p_pres.font.color.rgb = ACCENT_AMBER
        p_pres.alignment = PP_ALIGN.CENTER

    def add_card(slide, left, top, width, height, card_title, bullets, header_color=ACCENT_PURPLE):
        card = slide.shapes.add_shape(MSO_SHAPE.ROUNDED_RECTANGLE, Inches(left), Inches(top), Inches(width), Inches(height))
        card.fill.solid()
        card.fill.fore_color.rgb = CARD_BG
        card.line.color.rgb = CARD_BORDER
        card.line.width = Pt(1)

        tx_box = slide.shapes.add_textbox(Inches(left + 0.2), Inches(top + 0.15), Inches(width - 0.4), Inches(height - 0.3))
        tf = tx_box.text_frame
        tf.word_wrap = True

        if card_title:
            p_head = tf.paragraphs[0]
            p_head.text = card_title
            p_head.font.size = Pt(15)
            p_head.font.bold = True
            p_head.font.color.rgb = header_color
            p_head.space_after = Pt(8)

        for i, item in enumerate(bullets):
            p = tf.add_paragraph() if (card_title or i > 0) else tf.paragraphs[0]
            p.text = f"•  {item}"
            p.font.size = Pt(12)
            p.font.color.rgb = TEXT_WHITE
            p.space_after = Pt(6)

    # ==========================================
    # SLIDE 1: Cover & Team Introduction
    # ==========================================
    s1 = prs.slides.add_slide(blank_slide_layout)
    set_slide_background(s1)

    # Title Box
    title_box = s1.shapes.add_textbox(Inches(0.8), Inches(0.8), Inches(11.7), Inches(1.5))
    tf1 = title_box.text_frame
    p1 = tf1.paragraphs[0]
    p1.text = "Smart File Manager Pro"
    p1.font.size = Pt(36)
    p1.font.bold = True
    p1.font.color.rgb = ACCENT_PURPLE

    p1_sub = tf1.add_paragraph()
    p1_sub.text = "ชุดเครื่องมือจัดการไฟล์อัจฉริยะแบบ Local-First ปลอดภัย ทรงพลัง และรองรับภาษาไทย 100%"
    p1_sub.font.size = Pt(16)
    p1_sub.font.color.rgb = TEXT_MUTED
    p1_sub.space_before = Pt(8)

    # 4 Member Cards
    members = [
        ("ทัตพร สีโส", "69030093", "Lead Developer", "ดูแลแกนหลัก Core, DB SQLite, CI/CD, ความปลอดภัย"),
        ("ณัฐพงศ์ เหินหาว", "69030081", "UI Designer", "ออกแบบหน้าตาโปรแกรม UI/UX, ธีมสี, ระบบแปลภาษาไทย"),
        ("ภานุพงษ์ บำรุงศรี", "69030194", "Tester / QA", "วางแผนทดสอบ Test Plan, รัน 58 Tests, Regression Report"),
        ("นายปฏิภาณ พิพัฒน์พิทยากุล", "69030144", "System Architect & Docs", "ออกแบบสถาปัตยกรรม, Roadmap, ควบคุมเอกสารวิศวกรรม")
    ]
    card_w = 2.7
    gap = 0.3
    start_x = 0.8
    y = 2.8
    for idx, (name, sid, role, desc) in enumerate(members):
        cx = start_x + idx * (card_w + gap)
        m_card = s1.shapes.add_shape(MSO_SHAPE.ROUNDED_RECTANGLE, Inches(cx), Inches(y), Inches(card_w), Inches(3.8))
        m_card.fill.solid()
        m_card.fill.fore_color.rgb = CARD_BG
        m_card.line.color.rgb = ACCENT_PURPLE if idx == 0 else CARD_BORDER
        m_card.line.width = Pt(1.5)

        # Avatar placeholder box
        avatar = s1.shapes.add_shape(MSO_SHAPE.RECTANGLE, Inches(cx + 0.6), Inches(y + 0.3), Inches(1.5), Inches(1.5))
        avatar.fill.solid()
        avatar.fill.fore_color.rgb = RGBColor(45, 55, 90)
        avatar.line.color.rgb = ACCENT_PURPLE
        tf_av = avatar.text_frame
        p_av = tf_av.paragraphs[0]
        p_av.text = "[ รูปถ่าย ]"
        p_av.font.size = Pt(11)
        p_av.font.color.rgb = TEXT_MUTED
        p_av.alignment = PP_ALIGN.CENTER

        # Member Info
        info_box = s1.shapes.add_textbox(Inches(cx + 0.1), Inches(y + 1.9), Inches(card_w - 0.2), Inches(1.7))
        tf_info = info_box.text_frame
        tf_info.word_wrap = True

        p_name = tf_info.paragraphs[0]
        p_name.text = name
        p_name.font.size = Pt(13)
        p_name.font.bold = True
        p_name.font.color.rgb = TEXT_WHITE
        p_name.alignment = PP_ALIGN.CENTER

        p_sid = tf_info.add_paragraph()
        p_sid.text = f"รหัสนักศึกษา: {sid}"
        p_sid.font.size = Pt(11)
        p_sid.font.color.rgb = ACCENT_AMBER
        p_sid.alignment = PP_ALIGN.CENTER

        p_role = tf_info.add_paragraph()
        p_role.text = role
        p_role.font.size = Pt(11)
        p_role.font.bold = True
        p_role.font.color.rgb = ACCENT_GREEN
        p_role.alignment = PP_ALIGN.CENTER

        p_desc = tf_info.add_paragraph()
        p_desc.text = desc
        p_desc.font.size = Pt(10)
        p_desc.font.color.rgb = TEXT_MUTED
        p_desc.alignment = PP_ALIGN.CENTER
        p_desc.space_before = Pt(4)

    # ==========================================
    # SLIDE 2: Background & Motivation
    # ==========================================
    s2 = prs.slides.add_slide(blank_slide_layout)
    add_header(s2, "ความเป็นมาของโครงงาน (Background & Motivation)", "หัวข้อที่ 2", "นายปฏิภาณ พิพัฒน์พิทยากุล")

    add_card(s2, 0.8, 1.8, 5.6, 5.0, "ปัญหาในปัจจุบัน (Problem Statement)", [
        "ปัญหาไฟล์ขยะสะสม: โฟลเดอร์ Downloads และดิสก์ทำงานหนักจากไฟล์ชั่วคราวและไฟล์ซ้ำซ้อน",
        "ความเสี่ยงเรื่องความเป็นส่วนตัว: โปรแกรมจัดการไฟล์ทั่วไปมักส่งข้อมูล Telemetry และชื่อไฟล์ไปยัง Cloud ภายนอก",
        "ผู้ใช้ไม่กล้าลบไฟล์: กลัวข้อมูลสำคัญสูญหาย เพราะขาดระบบ Undo ที่ปลอดภัยและกู้คืนได้จริง",
        "การจัดระเบียบไฟล์ด้วยมือใช้เวลานาน: ขาดการจำแนกประเภทอัตโนมัติตามเดือนหรือเนื้อหาภาพถ่าย"
    ], ACCENT_PURPLE)

    add_card(s2, 6.8, 1.8, 5.7, 5.0, "แรงจูงใจในการพัฒนา (Motivation & Solution)", [
        "สถาปัตยกรรม Local-First: ข้อมูลไฟล์ทั้งหมดประมวลผลและเก็บในเครื่องของผู้ใช้ 100% ไม่รั่วไหลออกภายนอก",
        "ความปลอดภัยสูงสุด: การลบไฟล์กระทำผ่าน OS Recycle Bin เสมอ และป้องกันการแตะต้องโฟลเดอร์ระบบ",
        "ระบบกู้คืนข้ามเซสชัน (Persistent Undo): ใช้ฐานข้อมูล SQLite เก็บประวัติการย้ายและเปลี่ยนชื่อ ทำให้ย้อนกลับได้แม้รีสตาร์ต",
        "ประสิทธิภาพสูง: ใช้ SHA-256 แบบ Streaming พร้อม Hash Cache ทำให้การสแกนซ้ำเร็วขึ้น 3.6 เท่า"
    ], ACCENT_GREEN)

    # ==========================================
    # SLIDE 3: Objectives
    # ==========================================
    s3 = prs.slides.add_slide(blank_slide_layout)
    add_header(s3, "วัตถุประสงค์ของโครงงาน (Project Objectives)", "หัวข้อที่ 3", "นายปฏิภาณ พิพัฒน์พิทยากุล")

    add_card(s3, 0.8, 1.8, 3.6, 5.0, "1. ประสิทธิภาพและความปลอดภัย", [
        "พัฒนาโปรแกรมจัดการไฟล์แบบ Local-First ทำงานรวดเร็ว ไม่พึ่งพาคลาวด์สำหรับข้อมูลไฟล์",
        "ป้องกันข้อมูลสูญหายด้วยระบบ Safe Cleanup และปฏิเสธการแตะต้อง System Folders",
        "พัฒนาระบบ Hash Cache สำหรับ Duplicate Finder เพื่อประหยัดเวลาสแกนดิสก์ขนาดใหญ่"
    ], ACCENT_CYAN)

    add_card(s3, 4.8, 1.8, 3.6, 5.0, "2. ความเป็นระเบียบและระบบ Undo", [
        "สร้างระบบจัดระเบียบไฟล์อัตโนมัติ (File Organizer) ตามประเภท, วันที่, หรือกฎ Custom Rules",
        "จัดเก็บประวัติการทำงานด้วย SQLite Persistence เพื่อรองรับ Undo ข้ามการรีสตาร์ต",
        "พัฒนาระบบ Perceptual Photo Finder ค้นหารูปภาพที่ถ่ายต่อเนื่องหรือคล้ายกัน"
    ], ACCENT_PURPLE)

    add_card(s3, 8.8, 1.8, 3.7, 5.0, "3. มาตรฐานซอฟต์แวร์สากล", [
        "รองรับ 2 ภาษา (ไทยและอังกฤษ) ครอบคลุม 100% ทุกหน้าต่างและข้อความแจ้งเตือน",
        "ทดสอบระบบอัตโนมัติผ่าน Unit Tests 58 รายการ และ Headless CI Pipeline บน GitHub Actions",
        "สร้างตัวติดตั้งมาตรฐาน Windows Setup (.exe) และ macOS Disk Image (.dmg) พร้อมใช้งานทันที"
    ], ACCENT_GREEN)

    # ==========================================
    # SLIDE 4: Features Overview
    # ==========================================
    s4 = prs.slides.add_slide(blank_slide_layout)
    add_header(s4, "ขีดความสามารถของโครงงาน (Features Overview)", "หัวข้อที่ 4", "ณัฐพงศ์ เหินหาว")

    add_card(s4, 0.8, 1.8, 3.7, 2.4, "📊 Dashboard & Metrics", [
        "Tidiness Score Gauge (0-100) ประเมินความเป็นระเบียบ",
        "Sparklines วัด CPU, RAM, Network แบบ Real-time",
        "Quick Switcher (Ctrl+K) ค้นหาและสลับหน้าอย่างรวดเร็ว"
    ], ACCENT_PURPLE)

    add_card(s4, 4.8, 1.8, 3.7, 2.4, "📁 File Organizer", [
        "โหมดจัดระเบียบ: By type, By month, Type + Month",
        "รองรับ Custom Rules (เช่น *.psd -> Design)",
        "ระบบแสดงตัวอย่างผลลัพธ์ (Auto Preview) ก่อนดำเนินการจริง"
    ], ACCENT_CYAN)

    add_card(s4, 8.8, 1.8, 3.7, 2.4, "🔍 Duplicate & Photos", [
        "Duplicate Finder: Partial Hash -> Full SHA-256",
        "Similar Photos: Perceptual Hashing 3 รูปแบบ",
        "ระบบแสดงภาพย่อ (Thumbnails) ช่วยตัดสินใจลบ"
    ], ACCENT_GREEN)

    add_card(s4, 0.8, 4.5, 3.7, 2.3, "🧹 Safe Cleanup", [
        "ค้นหาไฟล์ชั่วคราว, โฟลเดอร์ว่าง, ไฟล์ใหญ่ที่ไม่ได้ใช้",
        "ส่งลง OS Recycle Bin เท่านั้น ไม่ลบถาวรทันที",
        "ปลอดภัย ไม่มีการติ๊กเลือกไฟล์สำคัญไว้ล่วงหน้า"
    ], ACCENT_AMBER)

    add_card(s4, 4.8, 4.5, 3.7, 2.3, "🏷️ Bulk Renamer & Backup", [
        "เปลี่ยนชื่อไฟล์เป็นชุด: Prefix, Suffix, Numbering",
        "ตรวจจับชื่อไฟล์ชนกันอัตโนมัติก่อนเปลี่ยนชื่อจริง",
        "Backup ทางเดียวเฉพาะไฟล์ใหม่/ที่เปลี่ยน (ไม่ลบไฟล์เดิม)"
    ], ACCENT_PURPLE)

    add_card(s4, 8.8, 4.5, 3.7, 2.3, "🎨 Themes & Bilingual UI", [
        "รองรับ 6 ธีมสี (Lavender, Obsidian, Deep Blue, etc.)",
        "ปรับขนาดฟอนต์ได้ รองรับฟอนต์ไทยคมชัด",
        "ภาษาไทยสมบูรณ์แบบ สลับภาษาได้ทันทีใน Settings"
    ], ACCENT_GREEN)

    # ==========================================
    # SLIDE 5: Demo & Screenshots
    # ==========================================
    s5 = prs.slides.add_slide(blank_slide_layout)
    add_header(s5, "ตัวอย่างของโครงงานที่ทำเสร็จแล้ว (Application Demo)", "หัวข้อที่ 4 (ต่อ)", "ทัตพร สีโส")

    add_card(s5, 0.8, 1.8, 5.6, 5.0, "ภาพรวมส่วนติดต่อผู้ใช้ (UI Highlights)", [
        "การออกแบบโมเดิร์น: แถบ Sidebar แบบ Slim Icons โทนม่วงอ่อน สบายตาและประหยัดพื้นที่",
        "หน้า Dashboard: สรุปคะแนน Tidiness Score วงแหวนสีสดใส แนะนำจุดที่ควรจัดระเบียบ",
        "ระบบค้นหาไฟล์ซ้ำ: แสดงรายการกลุ่มไฟล์ซ้ำ พร้อมจำนวนไบต์ที่ประหยัดได้หากทำความสะอาด",
        "กล่องโต้ตอบการจัดการบัญชี: ออกแบบรองรับภาษาไทยเต็มรูปแบบ แสดงข้อมูลอุปกรณ์และโปรไฟล์",
        "*(สามารถสาธิตโปรแกรมจริงประกอบสไลด์หน้านี้ได้ 2-3 นาที)*"
    ], ACCENT_PURPLE)

    # Demo Mockup Box
    demo_box = s5.shapes.add_shape(MSO_SHAPE.ROUNDED_RECTANGLE, Inches(6.8), Inches(1.8), Inches(5.7), Inches(5.0))
    demo_box.fill.solid()
    demo_box.fill.fore_color.rgb = RGBColor(28, 34, 58)
    demo_box.line.color.rgb = ACCENT_CYAN
    demo_box.line.width = Pt(1.5)
    tf_demo = demo_box.text_frame
    p_d1 = tf_demo.paragraphs[0]
    p_d1.text = "🖥️ [ วางภาพแคปหน้าจอโปรแกรมจริง ]\n(Dashboard / Organizer / Duplicate Finder)"
    p_d1.font.size = Pt(14)
    p_d1.font.bold = True
    p_d1.font.color.rgb = ACCENT_AMBER
    p_d1.alignment = PP_ALIGN.CENTER

    p_d2 = tf_demo.add_paragraph()
    p_d2.text = "\n• Release v4.3.1 เปิดใช้งานได้ทันทีทั้ง Windows และ macOS\n• สาธิต: การจัดไฟล์ Downloads ➔ ย้ายไฟล์ ➔ กด Undo กู้คืนกลับมาทันที"
    p_d2.font.size = Pt(12)
    p_d2.font.color.rgb = TEXT_WHITE

    # ==========================================
    # SLIDE 6: SDLC Model & Roles
    # ==========================================
    s6 = prs.slides.add_slide(blank_slide_layout)
    add_header(s6, "โมเดลการพัฒนาซอฟต์แวร์ และหน้าที่ตามโมเดล", "หัวข้อที่ 5", "นายปฏิภาณ พิพัฒน์พิทยากุล")

    add_card(s6, 0.8, 1.8, 5.6, 5.0, "โมเดลการพัฒนา (Agile + Gitflow)", [
        "โมเดล: Agile / Feature-Driven Development (FDD) แบ่งรอบการพัฒนาเป็น 9 Phases ชัดเจน",
        "การผสานงาน: ใช้ Gitflow Model แยก Branch ตามบทบาทหน้าที่อย่างเป็นสัดส่วน",
        "กิ่งหลัก: main (Production Stable) และ develop (Integration Base)",
        "Code Review & Verification: รวมโค้ดผ่าน Pull Request เท่านั้น และต้องผ่านการทดสอบ 100%",
        "Continuous Delivery: ผสาน CI/CD บน GitHub Actions บิลด์ตัวติดตั้งอัตโนมัติเมื่อติด Release Tag"
    ], ACCENT_PURPLE)

    add_card(s6, 6.8, 1.8, 5.7, 5.0, "หน้าที่ของสมาชิกแต่ละคนตามโมเดล", [
        "ทัตพร สีโส (Lead Developer): รับผิดชอบกิ่ง team/dev พัฒนา Core Logic, SQLite, แก้ไข Spec บิลด์ และระบบ Security/Perf",
        "ณัฐพงศ์ เหินหาว (UI Designer): รับผิดชอบกิ่ง team/ui-design ออกแบบส่วนติดต่อผู้ใช้ วิดเจ็ต ธีม และแปลภาษาไทย 100%",
        "ภานุพงษ์ บำรุงศรี (Tester / QA): รับผิดชอบกิ่ง team/tester ออกแบบ Test Plan, สร้างชุดทดสอบ 58 Unit Tests และ Regression Analysis",
        "นายปฏิภาณ พิพัฒน์พิทยากุล (Architect & Docs): รับผิดชอบกิ่ง team/docs วางสถาปัตยกรรมระบบ จัดทำ Roadmap และเอกสารทางเทคนิค"
    ], ACCENT_GREEN)

    # ==========================================
    # SLIDE 7: Overall Architecture
    # ==========================================
    s7 = prs.slides.add_slide(blank_slide_layout)
    add_header(s7, "โครงสร้างของโครงงานในภาพรวม (Overall Architecture)", "หัวข้อที่ 6", "นายปฏิภาณ พิพัฒน์พิทยากุล")

    add_card(s7, 0.8, 1.8, 3.7, 5.0, "1. Presentation Layer (UI)", [
        "PySide6 (Qt for Python): เฟรมเวิร์กหลักสำหรับหน้าต่างเดสก์ท็อป",
        "Responsive Card Layout: เลย์เอาต์แบบการ์ดปรับขนาดตามหน้าจอ",
        "Dynamic Theming: สลับ 6 ธีมสีแบบ On-the-fly โดยไม่ต้องรีสตาร์ต",
        "Bilingual Engine: จัดการข้อความ UI ผ่านโมดูล i18n_th แบบ Real-time"
    ], ACCENT_PURPLE)

    add_card(s7, 4.8, 1.8, 3.7, 5.0, "2. Application Services", [
        "QThread Concurrency: รันงานสแกนไฟล์และคำนวณ Hash เบื้องหลัง หน้าจอไม่ค้าง",
        "Streaming SHA-256 Engine: แบ่งอ่าน Buffer ขนาด 1MB คำนวณความเร็วสูง ~1.8 GB/s",
        "Perceptual Image Hash: คำนวณ aHash, dHash, pHash เปรียบเทียบภาพซ้ำ",
        "Safety Guard: ฟังก์ชัน is_protected() ดักจับ Path Traversal และโฟลเดอร์ระบบ"
    ], ACCENT_CYAN)

    add_card(s7, 8.8, 1.8, 3.7, 5.0, "3. Data & Platform Layer", [
        "Local SQLite (sfm.db): จัดเก็บ undo_batches, undo_moves, hash_cache, activity_log",
        "OS Recycle Bin: ลบไฟล์ผ่านไลบรารี send2trash ปลอดภัยกู้คืนได้",
        "Supabase Auth: บริการยืนยันตัวตนคลาวด์ทางเลือก (เก็บเฉพาะ User ID, ไม่เก็บไฟล์)",
        "Zero Telemetry: ไม่มีการส่งข้อมูลดิสก์ออกนอกเครื่อง รักษาความเป็นส่วนตัวสมบูรณ์แบบ"
    ], ACCENT_GREEN)

    # ==========================================
    # SLIDE 8: Code Overview - UI
    # ==========================================
    s8 = prs.slides.add_slide(blank_slide_layout)
    add_header(s8, "อธิบายโค้ดส่วน UI & Localization (Presentation Layer)", "หัวข้อที่ 7", "ณัฐพงศ์ เหินหาว")

    add_card(s8, 0.8, 1.8, 5.6, 5.0, "app.py, themes.py และ widgets.py", [
        "app.py: จุดเริ่มต้นโปรแกรม (main entry point) ควบคุมวงจรชีวิต MainWindow, เมนูบาร์, System Tray และรองรับโหมด --selftest",
        "themes.py: นิยามตารางสีและ QSS Stylesheet สำหรับทั้ง 6 ธีม รองรับการแสดงผลฟอนต์ภาษาไทย (Leelawadee UI)",
        "widgets.py: วิดเจ็ตเฉพาะทาง เช่น Sparkline กราฟิกแสดงการใช้ CPU/RAM, Ring Score Gauge และป๊อปอัปแจ้งเตือน Toast",
        "account_dialog.py: หน้าต่างจัดการโปรไฟล์ผู้ใช้และอุปกรณ์ที่ล็อกอิน"
    ], ACCENT_PURPLE)

    add_card(s8, 6.8, 1.8, 5.7, 5.0, "i18n.py และ i18n_th.py (ระบบสองภาษา)", [
        "ฟังก์ชัน tr(key): ทำหน้าที่ค้นหาคำแปลตามภาษาที่ผู้ใช้เลือก (Default เป็นภาษาอังกฤษ หากเลือกไทยจะแมปกับคลังคำแปล)",
        "i18n_th.py: เก็บ Dictionary คำแปลภาษาไทยมากกว่า 150 รายการ ครอบคลุมทั้งหน้าจอหลัก ไดอะล็อกยืนยันตัวตน และข้อความ Error",
        "Zero Hardcoded Strings: โค้ด UI ทั้งหมดถูก Refactor ให้ผ่าน tr(...) 100%",
        "test_i18n.py: ใช้โมดูล ast สแกนโค้ด Python ทุกบรรทัดเพื่อตรวจจับว่ามีข้อความ UI ใดตกหล่นไม่มีคำแปลภาษาไทยหรือไม่"
    ], ACCENT_CYAN)

    # ==========================================
    # SLIDE 9: Code Overview - Core & DB
    # ==========================================
    s9 = prs.slides.add_slide(blank_slide_layout)
    add_header(s9, "อธิบายโค้ดส่วน Core Logic & Database Persistence", "หัวข้อที่ 7 (ต่อ)", "ทัตพร สีโส")

    add_card(s9, 0.8, 1.8, 5.6, 5.0, "core.py — กลไกประมวลผลไฟล์ (Pure Python)", [
        "scan_directory(): ท่องไดเรกทอรีแบบ Generator ข้ามไฟล์ระบบและไฟล์ที่ถูกล็อกอย่างปลอดภัย",
        "find_duplicates(): สแกน 3 ชั้น: (1) เปรียบเทียบขนาดไฟล์ -> (2) Partial Hash 8KB แรก -> (3) Full SHA-256",
        "organize_files(): ย้ายไฟล์ตามหมวดหมู่นามสกุลไฟล์หรือเดือน โดยตรวจจับชื่อชนกันอัตโนมัติด้วย unique_dest()",
        "undo_move(): ดึงรายการย้ายจากฐานข้อมูลเพื่อคืนไฟล์กลับตำแหน่งเดิมอย่างแม่นยำ",
        "is_protected(): ป้องกันโฟลเดอร์ C:\\Windows, /System, /usr และ Drive roots"
    ], ACCENT_GREEN)

    add_card(s9, 6.8, 1.8, 5.7, 5.0, "db.py & membership_client.py", [
        "db.py (SQLite): ใช้ไลบรารีมาตรฐาน sqlite3 ไม่ต้องติดตั้งเพิ่ม จัดเก็บตาราง undo_batches, undo_moves, hash_cache, rules, activity_log",
        "Hash Cache: บันทึกค่า SHA-256 คู่กับ File Size และ mtime เมื่อสแกนซ้ำจะข้ามการอ่านไฟล์ ช่วยเพิ่มความเร็ว 3.6 เท่า",
        "membership_client.py: เชื่อมต่อ Supabase Auth ฝังค่าเริ่มต้น Default Public Credentials ทำให้ผู้ใช้เปิดโปรแกรมได้ทันทีโดยไม่ต้องตั้งค่า .env",
        "SQL Parameterization: คำสั่ง SQL ทุกตัวใช้พารามิเตอร์ ? ป้องกันช่องโหว่ SQL Injection"
    ], ACCENT_AMBER)

    # ==========================================
    # SLIDE 10: Code Overview - QA & CI/CD
    # ==========================================
    s10 = prs.slides.add_slide(blank_slide_layout)
    add_header(s10, "อธิบายโค้ดส่วนการทดสอบและ CI/CD (QA & DevOps)", "หัวข้อที่ 7 (ต่อ)", "ภานุพงษ์ บำรุงศรี")

    add_card(s10, 0.8, 1.8, 5.6, 5.0, "ชุดทดสอบอัตโนมัติ (tests/ — 58 Tests)", [
        "test_core.py (25 การทดสอบ): ตรวจสอบความปลอดภัย Path traversal, การย้ายไฟล์, การคำนวณคะแนน และ Perceptual photos",
        "test_db.py (7 การทดสอบ): ตรวจสอบความถูกต้องของ SQLite, Hash Cache, และการตัดประวัติที่หมดอายุ",
        "test_ui.py (19 การทดสอบ): จำลองการทำงานของหน้าจอ UI, การสลับธีม, การสร้างการ์ด และการคลิกปุ่มแบบ Headless",
        "test_i18n.py (4 การทดสอบ): ใช้ AST Scanner สแกนความครบถ้วนของคำแปลภาษาไทยในทุกไฟล์ UI",
        "test_email_callback.py (3 การทดสอบ): ตรวจสอบระบบ Loopback server สำหรับรับ Token ยืนยันอีเมล"
    ], ACCENT_PURPLE)

    add_card(s10, 6.8, 1.8, 5.7, 5.0, "CI/CD Pipeline (.github/workflows/)", [
        "ci.yml: เวิร์กโฟลว์ Matrix ทดสอบอัตโนมัติบน Ubuntu-latest และ Windows-latest ด้วย Python 3.12 ทุกครั้งที่มีการสร้าง Pull Request",
        "Headless Test: รันชุดทดสอบด้วย QT_QPA_PLATFORM=offscreen ทำให้รันบนเซิร์ฟเวอร์ CI ที่ไม่มีหน้าจอได้ 100%",
        "build-windows.yml: คอมไพล์โปรแกรมผ่าน PyInstaller และ Inno Setup 6 เพื่อสร้าง SmartFileManagerPro-Setup.exe",
        "Release Automation: อัปโหลดตัวติดตั้งเข้า GitHub Release อัตโนมัติเมื่อมีการ Push Tag เวอร์ชันใหม่"
    ], ACCENT_CYAN)

    # ==========================================
    # SLIDE 11: Problems & Solutions 1
    # ==========================================
    s11 = prs.slides.add_slide(blank_slide_layout)
    add_header(s11, "ปัญหาและแนวทางแก้ไข: UI/UX & การแปลภาษา", "หัวข้อที่ 8", "ณัฐพงศ์ เหินหาว")

    add_card(s11, 0.8, 1.8, 5.6, 5.0, "ปัญหาที่ 1: ภาษาอังกฤษตกค้างในไดอะล็อก", [
        "ปัญหา: หน้าต่าง AccountDialog และ AuthDialog มีข้อความภาษาอังกฤษ Hardcoded อยู่ในโค้ด ทำให้ผู้ใช้ที่เลือกภาษาไทยยังคงเห็นข้อความภาษาอังกฤษ",
        "สาเหตุ: โค้ดเดิมเขียนข้อความเป็นสตริงตรงๆ ไม่ได้เรียกใช้ฟังก์ชัน tr(...) ในโมดูล i18n",
        "แนวทางแก้ไข:",
        "  1. ทำการ Refactor ข้อความ UI ทั้งหมดใน account_dialog.py และ auth_dialog.py ให้ห่อหุ้มด้วย tr(...)",
        "  2. เพิ่มรายการคำแปลภาษาไทยใหม่จำนวน 35 รายการใน i18n_th.py",
        "  3. ขยายขอบเขต AST Translation Scanner ใน test_i18n.py ให้ครอบคลุมทุกไฟล์ไดอะล็อก เพื่อป้องกันการหลุดในอนาคต"
    ], ACCENT_PURPLE)

    add_card(s11, 6.8, 1.8, 5.7, 5.0, "ปัญหาที่ 2: ค่า QSettings รั่วไหลข้าม Unit Test", [
        "ปัญหา: การรันชุดทดสอบ test_ui.py บน macOS เกิดข้อผิดพลาดล้มเหลว 3 การทดสอบ (Failures) ทั้งที่บน Linux ผ่านทั้งหมด",
        "สาเหตุ: บน macOS คลาส QSettings เขียนลงไฟล์ Preferences ของระบบปฏิบัติการจริง (~/Library/Preferences/com.SmartFileManagerPro...) เมื่อเทสต์หนึ่งเปลี่ยนโหมดเป็น 'By month' ค่านี้จะค้างอยู่ในระบบ ทำให้เทสต์ถัดไปอ่านค่าผิดพลาด",
        "แนวทางแก้ไข:",
        "  1. ปรับแต่งเมธอด setUp() และ tearDown() ใน tests/test_ui.py ให้รีเซ็ตค่ากลับเป็นค่าเริ่มต้น 'By type' เสมอ",
        "  2. ผลลัพธ์: การทดสอบทั้ง 58 รายการผ่านฉลุย 100% บนทุกแพลตฟอร์ม"
    ], ACCENT_CYAN)

    # ==========================================
    # SLIDE 12: Problems & Solutions 2
    # ==========================================
    s12 = prs.slides.add_slide(blank_slide_layout)
    add_header(s12, "ปัญหาและแนวทางแก้ไข: Packaging & Runtime Setup", "หัวข้อที่ 8 (ต่อ)", "ทัตพร สีโส")

    add_card(s12, 0.8, 1.8, 5.6, 5.0, "ปัญหาที่ 3: ไฟล์ติดตั้งบิลด์ไม่พบฟอนต์และไอคอน", [
        "ปัญหา: เมื่อนำโค้ดไปบิลด์ผ่าน PyInstaller โปรแกรมเปิดไม่ขึ้นหรือแสดงผลฟอนต์ผิดเพี้ยน และขนาดไฟล์บิลด์ไม่ตรงกับที่ Inno Setup คาดหวัง",
        "สาเหตุ: ไฟล์ smartFileManagerPro.spec ขาดการระบุโฟลเดอร์ fonts/ และ icon.ico ในตัวแปร datas และ build_exe.bat ใช้คำสั่ง --onefile ซึ่งขัดแย้งกับ installer.iss",
        "แนวทางแก้ไข:",
        "  1. อัปเดต smartFileManagerPro.spec เพิ่ม fonts/ และ icon.ico เข้าไปใน datas tuple",
        "  2. ปรับปรุง build_exe.bat ให้เรียกใช้ spec file โดยตรง เพื่อสร้างโฟลเดอร์ผลลัพธ์ที่ตรงกับ Inno Setup อย่างสมบูรณ์"
    ], ACCENT_GREEN)

    add_card(s12, 6.8, 1.8, 5.7, 5.0, "ปัญหาที่ 4: ข้อผิดพลาด Supabase is not configured", [
        "ปัญหา: เมื่อผู้ใช้ดาวน์โหลดตัวติดตั้ง .exe หรือ .dmg ไปติดตั้งและเปิดใช้งาน โปรแกรมแจ้งเตือนว่าไม่พบการตั้งค่า Supabase และปิดตัวเองทันที",
        "สาเหตุ: ไฟล์ .env ถูกจัดให้อยู่ใน .gitignore ทำให้ตัวติดตั้งไม่มีไฟล์คอนฟิก และเครื่องผู้ใช้ยังไม่มีไฟล์ .env ในโฟลเดอร์ Home",
        "แนวทางแก้ไข:",
        "  1. ทำการฝังค่า Default Public Client Credentials ลงใน membership_client.py เป็นค่า Fallback อัตโนมัติ",
        "  2. อัปเดตตัวติดตั้งให้ทำงานแบบ Zero-Configuration Out-of-the-box ผู้ใช้เปิดโปรแกรมแล้วใช้งานได้ทันทีโดยไม่ต้องสร้างไฟล์ .env เอง"
    ], ACCENT_AMBER)

    # ==========================================
    # SLIDE 13: Practical Applications
    # ==========================================
    s13 = prs.slides.add_slide(blank_slide_layout)
    add_header(s13, "การนำไปประยุกต์ใช้งาน (Practical Applications)", "หัวข้อที่ 9", "ภานุพงษ์ บำรุงศรี")

    add_card(s13, 0.8, 1.8, 3.7, 5.0, "👥 ผู้ใช้งานทั่วไป (Everyday Users)", [
        "จัดระเบียบโฟลเดอร์ Downloads ที่รก: แยกประเภทเอกสาร รูปภาพ โปรแกรม วิดีโอ ได้ใน 1 คลิก",
        "ล้างไฟล์ขยะและโฟลเดอร์ว่าง: เพิ่มพื้นที่ว่างให้ฮาร์ดดิสก์อย่างปลอดภัย ไม่เสี่ยงไฟล์สำคัญหาย",
        "ผู้ใช้งานที่ไม่ชำนาญคอมพิวเตอร์: มีเมนูภาษาไทยเข้าใจง่าย และมีปุ่ม Undo กู้คืนการจัดไฟล์ได้ทันที"
    ], ACCENT_PURPLE)

    add_card(s13, 4.8, 1.8, 3.7, 5.0, "📸 ช่างภาพและคอนเทนต์ครีเอเตอร์", [
        "จัดหมวดหมู่ภาพถ่ายตามวันที่ (EXIF): จัดระเบียบภาพจากกล้องตามปี-เดือนที่กดถ่ายจริงโดยอัตโนมัติ",
        "Similar Photos: ตรวจจับภาพถ่ายรัว (Burst shots) หรือภาพที่บันทึกซ้ำซ้อนเพื่อเลือกเก็บภาพที่ดีที่สุด",
        "แสดงภาพย่อ (Thumbnails): เปรียบเทียบภาพที่คล้ายกันได้อย่างแม่นยำก่อนตัดสินใจส่งลงถังรีไซเคิล"
    ], ACCENT_CYAN)

    add_card(s13, 8.8, 1.8, 3.7, 5.0, "🏢 สำนักงานและนักพัฒนา (Office & Devs)", [
        "Bulk Renamer: เปลี่ยนชื่อเอกสาร สัญญา ใบเสร็จ หรือรูปภาพเป็นชุด พร้อมใส่เลขลำดับ 001, 002 ได้อย่างรวดเร็ว",
        "Local Backup: สำรองโฟลเดอร์งานสำคัญแบบทางเดียว ไม่เขียนทับไฟล์เดิม และไม่ลบข้อมูล",
        "ปลอดภัยสำหรับข้อมูลลับ: ไม่มี Telemetry ส่งเนื้อหาไฟล์ออกนอกบริษัท สอดคล้องตามมาตรฐาน PDPA"
    ], ACCENT_GREEN)

    # ==========================================
    # SLIDE 14: GitHub - ทัตพร สีโส
    # ==========================================
    s14 = prs.slides.add_slide(blank_slide_layout)
    add_header(s14, "การทำงานบน GitHub: ทัตพร สีโส (Lead Developer)", "หัวข้อที่ 10 (1/4)", "ทัตพร สีโส")

    add_card(s14, 0.8, 1.8, 4.5, 5.0, "บทบาทและกิ่งที่รับผิดชอบ", [
        "ชื่อสมาชิก: ทัตพร สีโส (รหัสนักศึกษา: 69030093)",
        "บทบาท: Lead Developer",
        "กิ่งหลักที่รับผิดชอบ: team/dev",
        "Task Branches ที่สร้างและทำงาน:",
        "  • fix/core-packaging-and-tests (Phase 3)",
        "  • sec-perf/audit-and-benchmarking (Phase 6)",
        "  • ci/automated-testing-workflow (Phase 7)",
        "  • fix/standalone-supabase-defaults (Phase 9/Hotfix)",
        "กิ่ง Integration ร่วม: develop ➔ main"
    ], ACCENT_PURPLE)

    add_card(s14, 5.6, 1.8, 6.9, 5.0, "ประวัติการ Commit และ Pull Request (ตั้งแต่วันแรก)", [
        "2026-11-09 00:00 - 00:06 | Baseline Commits (19392d0, 5bcd18d, 27be125, e748d40, 1347464)",
        "2026-11-10 21:31 | Commit a3267ff: fix: resolve packaging assets spec, headless selftest, and test isolation",
        "2026-11-10 21:32 | Pull Request #5 & #6: Merge fix/core-packaging-and-tests into develop",
        "2026-11-10 21:38 | Commit b245fb5: docs: add security policy, audit, and benchmark report",
        "2026-11-10 21:39 | Pull Request #11 & #12: Merge sec-perf into develop",
        "2026-11-10 21:40 | Commit 07ef947: ci: add multi-platform GitHub Actions testing workflow",
        "2026-11-10 21:41 | Pull Request #13 & #14: Merge ci/automated-testing-workflow into develop",
        "2026-11-10 22:25 | Commit 743ad61: fix: embed default Supabase credentials and bundle env in installer",
        "2026-11-10 22:26 | Pull Request #18, #19, #20: Merge v4.3.1 into main",
        "2026-11-10 22:29 | Commit 33b44cc: fix: make .env inclusion conditional in spec and remove hard dependency in iss"
    ], ACCENT_GREEN)

    # ==========================================
    # SLIDE 15: GitHub - ณัฐพงศ์ เหินหาว
    # ==========================================
    s15 = prs.slides.add_slide(blank_slide_layout)
    add_header(s15, "การทำงานบน GitHub: ณัฐพงศ์ เหินหาว (UI Designer)", "หัวข้อที่ 10 (2/4)", "ณัฐพงศ์ เหินหาว")

    add_card(s15, 0.8, 1.8, 4.5, 5.0, "บทบาทและกิ่งที่รับผิดชอบ", [
        "ชื่อสมาชิก: ณัฐพงศ์ เหินหาว (รหัสนักศึกษา: 69030081)",
        "บทบาท: UI Designer",
        "กิ่งหลักที่รับผิดชอบ: team/ui-design",
        "Task Branches ที่สร้างและทำงาน:",
        "  • feat/ui-localization-parity (Phase 4)",
        "ขอบเขตการทำงาน:",
        "  • ตรวจสอบส่วนติดต่อผู้ใช้ทุกหน้าจอ",
        "  • Refactor ข้อความ UI ให้รองรับ tr(...)",
        "  • เพิ่มคลังคำแปลภาษาไทยใน i18n_th.py",
        "  • จัดทำคู่มือ UI Guidelines ของระบบ"
    ], ACCENT_PURPLE)

    add_card(s15, 5.6, 1.8, 6.9, 5.0, "ประวัติการ Commit และ Pull Request", [
        "2026-11-10 21:35 | Commit 621f488:",
        "  'feat: complete Thai localization parity across auth and account dialogs'",
        "  - ห่อหุ้มข้อความใน account_dialog.py และ auth_dialog.py ด้วย tr(...)",
        "  - เพิ่มคำแปลภาษาไทย 35 รายการใน i18n_th.py สำหรับระบบบัญชีและอุปกรณ์",
        "  - แปลเมนูบาร์ส่วนของ Account Menu ใน app.py",
        "  - อัปเดต AST Scanner ใน tests/test_i18n.py ตรวจจับไดอะล็อกทั้งหมด",
        "2026-11-10 21:35 | Pull Request #7: feat/ui-localization-parity ➔ team/ui-design (Merged)",
        "2026-11-10 21:35 | Pull Request #8: team/ui-design ➔ develop (Merged)",
        "2026-11-10 21:45 | จัดทำเอกสาร docs/design/UI_GUIDELINES.md กำหนด Palette สีและฟอนต์"
    ], ACCENT_CYAN)

    # ==========================================
    # SLIDE 16: GitHub - ภานุพงษ์ บำรุงศรี
    # ==========================================
    s16 = prs.slides.add_slide(blank_slide_layout)
    add_header(s16, "การทำงานบน GitHub: ภานุพงษ์ บำรุงศรี (Tester / QA)", "หัวข้อที่ 10 (3/4)", "ภานุพงษ์ บำรุงศรี")

    add_card(s16, 0.8, 1.8, 4.5, 5.0, "บทบาทและกิ่งที่รับผิดชอบ", [
        "ชื่อสมาชิก: ภานุพงษ์ บำรุงศรี (รหัสนักศึกษา: 69030194)",
        "บทบาท: Tester / QA",
        "กิ่งหลักที่รับผิดชอบ: team/tester",
        "Task Branches ที่สร้างและทำงาน:",
        "  • test/full-qa-validation (Phase 5)",
        "ขอบเขตการทำงาน:",
        "  • จัดทำแผนการทดสอบ (Test Plan)",
        "  • ควบคุมและรันชุดทดสอบ 58 Unit Tests",
        "  • ตรวจสอบความเข้ากันได้แบบ Headless",
        "  • จัดทำรายงานผลการทดสอบและการถดถอย"
    ], ACCENT_GREEN)

    add_card(s16, 5.6, 1.8, 6.9, 5.0, "ประวัติการ Commit และ Pull Request", [
        "2026-11-10 21:37 | Commit 4128635:",
        "  'test: add comprehensive test plan, test report, and regression analysis'",
        "  - จัดทำเอกสาร docs/QA/TEST_PLAN.md กำหนด Test Matrix ครอบคลุม 5 โมดูล",
        "  - รันการทดสอบ Unit Tests ทั้ง 58 รายการ ผลการทดสอบผ่าน 100% (2.55s)",
        "  - จัดทำรายงาน docs/QA/TEST_REPORT.md สรุปผลการรันจริงแบบละเอียด",
        "  - จัดทำรายงาน docs/QA/REGRESSION_REPORT.md ยืนยัน Zero Regressions",
        "2026-11-10 21:37 | Pull Request #9: test/full-qa-validation ➔ team/tester (Merged)",
        "2026-11-10 21:37 | Pull Request #10: team/tester ➔ develop (Merged)",
        "2026-11-10 21:50 & 22:26 | ตรวจสอบ Verification Gate ก่อนการ Merge สู่ main ทั้ง v4.3.0 และ v4.3.1"
    ], ACCENT_AMBER)

    # ==========================================
    # SLIDE 17: GitHub - นายปฏิภาณ พิพัฒน์พิทยากุล
    # ==========================================
    s17 = prs.slides.add_slide(blank_slide_layout)
    add_header(s17, "การทำงานบน GitHub: นายปฏิภาณ พิพัฒน์พิทยากุล (Docs & Architect)", "หัวข้อที่ 10 (4/4)", "นายปฏิภาณ พิพัฒน์พิทยากุล")

    add_card(s17, 0.8, 1.8, 4.5, 5.0, "บทบาทและกิ่งที่รับผิดชอบ", [
        "ชื่อสมาชิก: นายปฏิภาณ พิพัฒน์พิทยากุล (รหัสนักศึกษา: 69030144)",
        "บทบาท: Documentation & System Architect",
        "กิ่งหลักที่รับผิดชอบ: team/docs",
        "Task Branches ที่สร้างและทำงาน:",
        "  • docs/initial-audit-and-roadmap (Phase 1)",
        "  • docs/architecture-and-guides (Phase 2)",
        "  • docs/comprehensive-docs-update (Phase 8)",
        "ขอบเขตการทำงาน:",
        "  • ออกแบบสถาปัตยกรรมระบบ Mermaid Diagram",
        "  • วางแผนและติดตาม Roadmap TSK-01 ถึง TSK-31",
        "  • จัดทำเอกสาร Handoff, Phase Log, Changelog"
    ], ACCENT_PURPLE)

    add_card(s17, 5.6, 1.8, 6.9, 5.0, "ประวัติการ Commit และ Pull Request", [
        "2026-11-10 21:25 | Commit dacf87e: docs: complete Phase 1 initial audit and development roadmap",
        "2026-11-10 21:26 | Pull Request #1 & #2: docs/initial-audit-and-roadmap ➔ develop (Merged)",
        "2026-11-10 21:28 | Commit cb61137: docs: add system architecture and developer guides",
        "2026-11-10 21:29 | Pull Request #3 & #4: docs/architecture-and-guides ➔ develop (Merged)",
        "2026-11-10 21:45 | Commit bf9f237: docs: complete Phase 8 comprehensive documentation and project status",
        "2026-11-10 21:45 | Pull Request #15: docs/comprehensive-docs-update ➔ team/docs (Merged)",
        "2026-11-10 21:46 | Pull Request #16: team/docs ➔ develop (Merged)",
        "เอกสารที่ส่งมอบ: ARCHITECTURE.md, INITIAL_AUDIT.md, ROADMAP.md, HANDOFF.md, CHANGELOG.md"
    ], ACCENT_CYAN)

    # ==========================================
    # SLIDE 18: Summary & Q&A
    # ==========================================
    s18 = prs.slides.add_slide(blank_slide_layout)
    add_header(s18, "สรุปผลการดำเนินงาน และช่วงถาม-ตอบ (Summary & Q&A)", "สรุปภาพรวม", "สมาชิกทุกคนในกลุ่ม")

    add_card(s18, 0.8, 1.8, 5.6, 5.0, "สรุปผลผลิตของโครงงาน (Deliverables)", [
        "ซอฟต์แวร์เสร็จสมบูรณ์: Smart File Manager Pro v4.3.1 พร้อมใช้งานจริงแบบ Out-of-the-box",
        "ตัวติดตั้งทางการ: มีให้ดาวน์โหลดทั้ง Windows (.exe) และ macOS (.dmg) บน GitHub Releases",
        "คุณภาพระดับสากล: ผ่านการทดสอบ Unit Tests 58/58 รายการ และมีระบบ CI/CD บน GitHub Actions",
        "เอกสารครบวงจร: ครอบคลุมทั้งสถาปัตยกรรม, แผนการทดสอบ, นโยบายความปลอดภัย, และคู่มือนักพัฒนา",
        "Source Code: เผยแพร่บน GitHub: github.com/69030093-thatphon/Smart-Filee"
    ], ACCENT_GREEN)

    # Q&A Card
    qa_card = s18.shapes.add_shape(MSO_SHAPE.ROUNDED_RECTANGLE, Inches(6.8), Inches(1.8), Inches(5.7), Inches(5.0))
    qa_card.fill.solid()
    qa_card.fill.fore_color.rgb = RGBColor(28, 34, 58)
    qa_card.line.color.rgb = ACCENT_PURPLE
    qa_card.line.width = Pt(2)
    tf_qa = qa_card.text_frame

    p_q1 = tf_qa.paragraphs[0]
    p_q1.text = "🎯 ขอบคุณครับ / ขอบคุณค่ะ"
    p_q1.font.size = Pt(26)
    p_q1.font.bold = True
    p_q1.font.color.rgb = ACCENT_AMBER
    p_q1.alignment = PP_ALIGN.CENTER

    p_q2 = tf_qa.add_paragraph()
    p_q2.text = "\nขอเปิดช่วงรับฟังข้อเสนอแนะ\nและตอบข้อซักถามจากท่านอาจารย์ (Q&A)"
    p_q2.font.size = Pt(16)
    p_q2.font.color.rgb = TEXT_WHITE
    p_q2.alignment = PP_ALIGN.CENTER

    p_q3 = tf_qa.add_paragraph()
    p_q3.text = "\n\nกลุ่มพัฒนา Smart File Manager Pro\nสถาบันเทคโนโลยีพระจอมเกล้าเจ้าคุณทหารลาดกระบัง (KMITL)"
    p_q3.font.size = Pt(12)
    p_q3.font.color.rgb = TEXT_MUTED
    p_q3.alignment = PP_ALIGN.CENTER

    output_path = "presentation/Smart_File_Manager_Pro_Presentation.pptx"
    prs.save(output_path)
    print(f"Presentation saved successfully to: {output_path}")

if __name__ == "__main__":
    create_presentation()
