from dotenv import load_dotenv
import slackweb
import random
import os


def main():
    load_dotenv()
    incoming_webhook_url = os.getenv('INCOMING_WEBHOOK_URL')
    slack_client = slackweb.Slack(incoming_webhook_url)
    m = [f':mahjong-man{i}:' for i in range(1, 10)]
    p = [f':mahjong-pin{i}:' for i in range(1, 10)]
    s = [f':mahjong-sou{i}:' for i in range(1, 10)]
    j = [':mahjong-ton:', ':mahjong-nan:', ':mahjong-sha:', ':mahjong-pei:',
         ':mahjong-haku:', ':mahjong-hatsu:', ':mahjong-chun:']
    mountain = s * 4 + m * 4 + p * 4 + j * 4
    haipai = random.sample(mountain, 13)

    slack_client.notify(
        text=f"今日の配牌:mahjong:\n{haipai}",
    )
    print(haipai)


if __name__ == '__main__':
    main()
