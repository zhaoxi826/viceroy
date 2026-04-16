#  // Copyright 2026 zhaoxi826
#  //
#  // Licensed under the Apache License, Version 2.0 (the "License");
#  // you may not use this file except in compliance with the License.
#  // You may obtain a copy of the License at
#  //
#  //     http://www.apache.org/licenses/LICENSE-2.0
#  //
#  // Unless required by applicable law or agreed to in writing, software
#  // distributed under the License is distributed on an "AS IS" BASIS,
#  // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  // See the License for the specific language governing permissions and
#  // limitations under the License.

import asyncio
import viceroy
import os

async def test_install():
    print("开始异步安装测试...")

    # 填入你的测试参数
    url = "https://github.com/anthropics/skills"
    path = "skills/docx"  # 仓库内的相对路径
    output = "./test_output"       # 最终输出目录

    try:
        # 验证是否返回了协程对象并可以 await
        # 这里的 install_skill 是你在 lib.rs 中定义的 async fn
        result_path = await viceroy.install_skill_async(url, path=path, output=output)

        print(f"✅ 安装成功！路径: {result_path}")

        # 检查文件是否生成
        if os.path.exists(os.path.join(result_path, "skill.json")):
            print("✅ skill.json 已生成")
        if os.path.exists(os.path.join(result_path, "metadata.json")):
            print("✅ metadata.json 已生成")

    except Exception as e:
        print(f"❌ 测试失败: {e}")

if __name__ == "__main__":
    asyncio.run(test_install())