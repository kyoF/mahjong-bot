from dotenv import load_dotenv
import slackweb
import random
import os


def main():
    load_dotenv()
    incoming_webhook_url = os.getenv('INCOMING_WEBHOOK_URL')
    slack_client = slackweb.Slack(incoming_webhook_url)
    s = [f':mahjong-sou{i}:' for i in range(1, 10)]
    m = [f':mahjong-man{i}:' for i in range(1, 10)]
    p = [f':mahjong-pin{i}:' for i in range(1, 10)]
    j = [':mahjong-ton:', ':mahjong-nan:', ':mahjong-sha:', 'mahjong-pei',
         'mahjong-haku', 'mahjong-hatsu', 'mahjong-chun']
    mountain = s * 4 + m * 4 + p * 4 + j * 4
    haipai = random.sample(mountain, 13)
    haipai.sort(
        key=lambda x: (
            x[1:].isdigit(), x[1], x[0] if x[0].isdigit() else float('inf')
        )
    )

    slack_text = []
    slack_text.append(
        {
            'blocks': [
                {
                    'type': 'section',
                    'text': {
                            'type': 'mrkdwn',
                            'text': 'aaa'
                    }
                }
            ]
        }
    )

    slack_client.notify(
        text="今日の配牌:mahjong:",
        attachments=slack_text
    )
    print(haipai)


if __name__ == '__main__':
    main()
