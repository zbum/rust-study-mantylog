# mantylog — Rust 학습 프로젝트

`mantylog`는 Rust를 기초부터 배우면서 한 단계씩 만들어가는 **로그 분석 CLI**입니다.
다른 머신/세션에서 이어서 학습할 때 이 문서를 먼저 읽으세요.

---

## 학습 워크플로우 (Claude와 협업)

- **사용자가 모든 학습 코드를 직접 타이핑**합니다. Claude는 `src/**`를 직접 편집하지 않습니다.
- Claude의 역할: 개념 설명, 실습 과제 제시, 작성된 코드 리뷰, 컴파일 에러 해석
- 사용자의 역할: 코드 작성, `cargo run` / `cargo check` 실행, 결과/에러 공유
- 스캐폴딩(`cargo new`, 의존성 추가, 샘플 데이터 파일 등)은 Claude가 도와줘도 OK

이어서 시작할 때 Claude에게 보낼 한 줄:
> "LEARNING.md 읽고 Step N부터 이어서 학습 진행하자."

---

## 커리큘럼 (8단계)

| Step | 주제 | 핵심 개념 | 상태 |
|------|------|----------|------|
| 1 | 프로젝트 셋업 + Hello World | `cargo new`, `cargo run`, `println!` 매크로, edition | ✅ 완료 |
| 2 | 변수·타입·함수·제어문 | `let`/`mut`, 기본 타입, 표현식 vs 문장, `if`가 표현식 | ✅ 완료 |
| 3 | Ownership & Borrowing | 소유권 3대 규칙, move, `&`/`&mut`, NLL, `String` vs `&str` | ✅ 완료 |
| 4 | struct·enum·match + Option/Result | 도메인 모델링, `match` exhaustiveness, `impl` 블록, `?` 연산자 | ✅ 완료 |
| 5 | 파일 IO + 컬렉션 | `fs::read_to_string`, `?` 본격 사용, `HashMap`, `entry().or_insert()` | ✅ 완료 |
| 6 | CLI 인자 파싱 (clap) | `cargo add`, `clap` derive API, `ValueEnum`, doc comments | ✅ 완료 |
| 7 | 트레이트 + 모듈 분리 | `trait`/`impl`, `mod`로 코드 분리, `From`/`Into`, `Display` | ✅ 완료 |
| 8 | JSON 출력 + 마무리 | `serde`/`serde_json`, 통합 + 리팩토링 | 🟡 다음 단계 |

---

## 현재 진도 (Step 8 시작 전)

### 동작하는 기능
- `sample.log` 파일을 읽어 라인별 레벨 분류
- `LogLevel` enum (`Error`/`Warn`/`Info`/`Unknown`) + `Display` 구현
- `HashMap<LogLevel, u32>`로 카운팅
- CLI 인자: `-i/--input <PATH>`, `-f/--filter <LEVEL>`
- `--help`, `--version` 자동 생성 (clap)
- 4개 모듈 분리: `main.rs` / `cli.rs` / `level.rs` / `parse.rs`
- `LevelArg → LogLevel` 변환은 표준 `From` 트레이트로 표현

### 사용 예시
```bash
cargo run                              # 기본 sample.log 전체 분석
cargo run -- --filter error            # ERROR 라인만 출력 (카운트는 전체)
cargo run -- -i other.log -f warn      # 다른 파일 + WARN 필터
cargo run -- --help                    # 도움말
```

---

## Step 8 미리보기 — JSON 출력 + 마무리 (마지막 단계)

- `cargo add serde --features derive` + `cargo add serde_json`
- `#[derive(Serialize)]`로 구조체/enum을 JSON으로 자동 변환
- 새 CLI 옵션 `--format text|json` (기본 `text`)
- 텍스트/JSON 출력 분기 (또는 trait로 다형 처리)
- 파이프 연결: `mantylog -f error --format json | jq ...` 같은 활용
- 마지막 정리 — README 추가, 코드 클린업

---

## 디렉토리 구조 (현재)

```
mantylog/
├── Cargo.toml
├── Cargo.lock
├── .gitignore           # /target, /.omc/, .idea/, *.iml, .DS_Store
├── LEARNING.md          # 이 문서
├── sample.log           # 학습용 샘플 로그 (15줄)
└── src/
    ├── main.rs          # 진입점, 글루 코드
    ├── cli.rs           # Args, LevelArg
    ├── level.rs         # LogLevel + From + Display + parse_log_level
    └── parse.rs         # first_token
```

---

## 새 머신에서 시작하기

```bash
git clone https://github.com/zbum/rust-study-mantylog.git
cd rust-study-mantylog
cargo run                # 첫 빌드 시 의존성 다운로드
```

필요 환경: Rust 1.95+ (`rustup install stable`로 설치)
