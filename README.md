Dưới đây là nội dung tệp README.md được thiết kế chuyên nghiệp, phù hợp để bạn đưa lên GitHub hoặc làm tài liệu thuyết trình cho dự án School Loyalty.

🎓 EduChain Token (ECT) – School Loyalty System
Khẩu hiệu: Đi học chuyên cần – Nhận phần thưởng số.

💡 Ý tưởng dự án
EduChain Token là một hệ thống lòng trung thành (Loyalty) dựa trên Blockchain Stellar dành cho môi trường giáo dục. Thay vì chỉ điểm danh truyền thống, mỗi buổi tham dự bài giảng của sinh viên sẽ được chuyển hóa thành tài sản số (Tokens).

Sinh viên không chỉ tích lũy điểm số, mà còn tích lũy "giá trị kinh tế" thực tế trong khuôn viên trường, tạo động lực mạnh mẽ để nâng cao tỷ lệ tham gia lớp học.

🔴 Vấn đề hiện nay
Hệ thống điểm danh hiện tại thường mang tính đối phó, thiếu minh bạch và không tạo ra giá trị gia tăng tức thời cho sinh viên, dẫn đến tình trạng bùng học hoặc nhờ người điểm danh hộ.

🟢 Giải pháp (Stellar Soroban)
Sử dụng Smart Contract (Soroban) trên mạng lưới Stellar để:

Tự động hóa: Giảng viên chỉ cần một lệnh gọi hàm để phân phối token cho toàn bộ lớp học.

Minh bạch: Mọi giao dịch thưởng và đổi quà đều được ghi lại trên sổ cái (Ledger), không thể tẩy xóa.

Chi phí cực thấp: Phí giao dịch trên Stellar gần như bằng 0, phù hợp với quy mô hàng ngàn giao dịch mỗi ngày trong trường học.

🛠 Tính năng cốt lõi (MVP)
Attendance Mining: Sinh viên nhận được ECT Token mỗi khi hoàn thành một buổi học (được xác thực bởi giảng viên).

Student Wallet: Mỗi sinh viên có một định danh (Identity/Address) riêng để quản lý số dư token của mình.

Redeem Rewards: Đổi token lấy các đặc quyền như:

Voucher giảm giá tại Căng-tin/Tiệm cà phê trong trường.

Quyền truy cập vào các khóa học chuyên sâu hoặc tài liệu thư viện VIP.

Điểm cộng ưu tiên khi đăng ký ký túc xế hoặc học bổng.

🏗 Công nghệ sử dụng
Blockchain: Stellar Network (Testnet).

Smart Contract: Soroban (Rust SDK).

CLI: Stellar CLI để tương tác và quản lý ví.

Dự kiến tích hợp: Web Dashboard hoặc Mobile App cho sinh viên.
