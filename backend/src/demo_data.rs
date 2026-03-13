use serde_json::{json, Value};

pub fn dashboard() -> Value {
    json!({
        "family_name": "示例家庭",
        "stats": {
            "people_count": 4,
            "upcoming_events": 3,
            "pending_tasks": 5,
            "things_count": 28,
            "spaces_count": 6
        },
        "happiness_score": 78,
        "recent_activities": [
            {"type": "task", "content": "完成家庭旅行计划", "time": "2小时前"},
            {"type": "event", "content": "女儿钢琴课", "time": "昨天"},
            {"type": "thing", "content": "新增洗碗机", "time": "3天前"}
        ]
    })
}

pub fn people() -> Vec<Value> {
    vec![
        json!({
            "id": "p001",
            "name": "张明",
            "role": "父亲",
            "birthday": "1980-06-15",
            "phone": "138****8888",
            "hobbies": ["读书", "爬山", "摄影"],
            "notes": "喜欢周末户外活动"
        }),
        json!({
            "id": "p002",
            "name": "李芳",
            "role": "母亲",
            "birthday": "1983-03-22",
            "phone": "139****9999",
            "hobbies": ["烹饪", "瑜伽", "园艺"],
            "notes": "负责家庭日常安排"
        }),
        json!({
            "id": "p003",
            "name": "张晓雨",
            "role": "女儿",
            "birthday": "2010-09-01",
            "phone": "",
            "hobbies": ["钢琴", "绘画", "阅读"],
            "notes": "小学五年级，成绩优秀"
        }),
        json!({
            "id": "p004",
            "name": "张小明",
            "role": "儿子",
            "birthday": "2014-12-18",
            "phone": "",
            "hobbies": ["乐高", "游泳", "足球"],
            "notes": "小学二年级，活泼好动"
        }),
    ]
}

pub fn events() -> Vec<Value> {
    vec![
        json!({
            "id": "e001",
            "title": "张晓雨钢琴课",
            "category": "教育",
            "start_time": "2026-03-14T15:00:00",
            "end_time": "2026-03-14T16:30:00",
            "location": "音乐学院",
            "participants": ["张晓雨", "李芳"],
            "recurring": "每周五"
        }),
        json!({
            "id": "e002",
            "title": "家庭春游",
            "category": "出行",
            "start_time": "2026-04-05T08:00:00",
            "end_time": "2026-04-05T20:00:00",
            "location": "郊外公园",
            "participants": ["张明", "李芳", "张晓雨", "张小明"],
            "recurring": null
        }),
        json!({
            "id": "e003",
            "title": "张小明游泳课",
            "category": "运动",
            "start_time": "2026-03-15T10:00:00",
            "end_time": "2026-03-15T11:30:00",
            "location": "社区游泳馆",
            "participants": ["张小明", "张明"],
            "recurring": "每周六"
        }),
        json!({
            "id": "e004",
            "title": "家庭医疗检查",
            "category": "健康",
            "start_time": "2026-03-20T09:00:00",
            "end_time": "2026-03-20T12:00:00",
            "location": "社区医院",
            "participants": ["张明", "李芳"],
            "recurring": "每年"
        }),
        json!({
            "id": "e005",
            "title": "外婆生日聚餐",
            "category": "家庭",
            "start_time": "2026-03-28T18:00:00",
            "end_time": "2026-03-28T21:00:00",
            "location": "家庭聚会",
            "participants": ["全家"],
            "recurring": "每年"
        }),
    ]
}

pub fn tasks() -> Vec<Value> {
    vec![
        json!({
            "id": "t001",
            "title": "预订春游门票",
            "priority": "高",
            "status": "进行中",
            "due_date": "2026-03-20",
            "assignee": "张明",
            "category": "出行"
        }),
        json!({
            "id": "t002",
            "title": "购买张晓雨教材",
            "priority": "中",
            "status": "待办",
            "due_date": "2026-03-18",
            "assignee": "李芳",
            "category": "教育"
        }),
        json!({
            "id": "t003",
            "title": "更换客厅灯泡",
            "priority": "低",
            "status": "待办",
            "due_date": "2026-03-22",
            "assignee": "张明",
            "category": "家务"
        }),
        json!({
            "id": "t004",
            "title": "整理书房",
            "priority": "低",
            "status": "待办",
            "due_date": "2026-03-30",
            "assignee": "全家",
            "category": "家务"
        }),
        json!({
            "id": "t005",
            "title": "缴纳水电费",
            "priority": "高",
            "status": "已完成",
            "due_date": "2026-03-10",
            "assignee": "李芳",
            "category": "财务"
        }),
        json!({
            "id": "t006",
            "title": "给植物浇水",
            "priority": "中",
            "status": "已完成",
            "due_date": "2026-03-12",
            "assignee": "张晓雨",
            "category": "家务"
        }),
    ]
}

pub fn things() -> Vec<Value> {
    vec![
        json!({"id": "th001", "name": "冰箱", "category": "家电", "location": "厨房", "purchase_date": "2022-06-15", "warranty_until": "2027-06-15", "notes": "海尔三门冰箱"}),
        json!({"id": "th002", "name": "洗碗机", "category": "家电", "location": "厨房", "purchase_date": "2026-03-10", "warranty_until": "2029-03-10", "notes": "西门子嵌入式"}),
        json!({"id": "th003", "name": "钢琴", "category": "乐器", "location": "客厅", "purchase_date": "2020-09-01", "warranty_until": null, "notes": "YAMAHA 立式钢琴"}),
        json!({"id": "th004", "name": "自行车×4", "category": "交通", "location": "车库", "purchase_date": "2023-04-10", "warranty_until": null, "notes": "全家出行用"}),
        json!({"id": "th005", "name": "空调×3", "category": "家电", "location": "各卧室+客厅", "purchase_date": "2021-07-20", "warranty_until": "2027-07-20", "notes": "格力变频，年度保养中"}),
        json!({"id": "th006", "name": "笔记本电脑", "category": "电子", "location": "书房", "purchase_date": "2024-01-05", "warranty_until": "2026-01-05", "notes": "张明工作用"}),
        json!({"id": "th007", "name": "平板电脑", "category": "电子", "location": "客厅", "purchase_date": "2023-08-15", "warranty_until": "2025-08-15", "notes": "孩子学习用，保修已过期"}),
    ]
}

pub fn spaces() -> Vec<Value> {
    vec![
        json!({"id": "s001", "name": "主卧", "type": "卧室", "area": "18㎡", "description": "张明和李芳的卧室，朝南，阳光充足"}),
        json!({"id": "s002", "name": "次卧（女儿房）", "type": "卧室", "area": "12㎡", "description": "张晓雨的房间，放置钢琴和书桌"}),
        json!({"id": "s003", "name": "次卧（儿子房）", "type": "卧室", "area": "10㎡", "description": "张小明的房间，有乐高收纳架"}),
        json!({"id": "s004", "name": "客厅", "type": "公共区域", "area": "32㎡", "description": "家庭活动中心，有大屏电视和沙发"}),
        json!({"id": "s005", "name": "厨房", "type": "功能区", "area": "10㎡", "description": "开放式厨房，配备齐全家电"}),
        json!({"id": "s006", "name": "书房", "type": "工作区", "area": "8㎡", "description": "张明的工作空间，有书架和电脑"}),
    ]
}
