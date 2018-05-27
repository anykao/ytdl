# _*_  coding:utf-8 _*_
import requests
import json
import re
import time
# aweme_ids =["6528693613306580228,","6415145228281318657","6534180318348315908","6543060223299423495","6516035207366184196",]
# aweme_ids =["6528693613306580228"]
aweme_ids = ['6534180318348315908']
# 获取三十六条数据
# url = "https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&cursor=0&count=36&parent_rid=20180411175245010008060232058BF3&aweme_id=6512598987021749508"
# 去掉参数aweme_id也是获取32
# url = "https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&cursor=0&count=36&parent_rid=20180411175245010008060232058BF3"
# 也是获取36
# url = "https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&cursor=0&count=36"

# about_media1
# url = "https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&count=36"
# about_media1
# url ="https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&cursor=0&count=36&parent_rid=20180411174917010011048231161F5B&aweme_id=6542452586539126029"
# url ="https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&cursor=0&count=36&parent_rid=20180411175245010008060232058BF3&aweme_id={}"
headers = {
    "User-Agent": "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/65.0.3325.181 Mobile Safari/537.36"}

i = 6

# ceshi


for aweme_id in aweme_ids:
    # i += 1
    url = "https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&cursor=0&count=36&parent_rid=20180411175245010008060232058BF3&aweme_id={}"
    # url = "https://www.douyin.com/aweme/v1/hot_aweme/?app_id=1128&cursor=0&count=36&parent_rid=20180411175245010008060232058BF3"

    print(aweme_id)
    url = url.format(aweme_id)
    # time.sleep(2)
    print(url)
    response = requests.get(url, headers=headers)
    res = response.content
    print(res)
