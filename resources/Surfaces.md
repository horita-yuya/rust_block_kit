# Surfaces

[Official doc](https://api.slack.com/surfaces)

## **[Messages](https://api.slack.com/surfaces/messages)**

```rust
let blocks = vec![
    Section(SectionBlock::new(Plain("Hello world".into()))),
    Context(ContextBlock::new(vec![
        TextContext(Markdown("*Markdown*".into()))
    ]))
];
let payload = ChatPostMessagePayload::new(
    "random",
    CommonMessagePayload::new().blocks(blocks),
)
    .icon_emoji(":computer:")
    .username("BOT");

// {
//   "channel": "random",
//   "blocks": [
//     {
//       "type": "section",
//       "text": {
//         "type": "plain_text",
//         "text": "Hello world"
//       }
//     },
//     {
//       "type": "context",
//       "elements": [
//         {
//           "type": "mrkdwn",
//           "text": "*Markdown*"
//         }
//       ]
//     }
//   ],
//   "icon_emoji": ":computer:",
//   "username": "BOT"
// }
let json = serde_json::to_string_pretty(&payload).unwrap();
```

## **[Modals](https://api.slack.com/surfaces/modals)**
*Comming soon*
## **[Home tab](https://api.slack.com/surfaces/tabs)**
*Comming soon* 