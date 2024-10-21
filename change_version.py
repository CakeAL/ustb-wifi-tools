import re

def update_version_with_regex(file_path, new_version):
    """使用正则表达式更新文件中的版本号"""
    with open(file_path, 'r', encoding='utf-8') as file:
        content = file.read()

    # 匹配并替换版本号
    updated_content = re.sub(r'"version"\s*:\s*"\d+\.\d+\.\d+"', f'"version": "{new_version}"', content)
    updated_content = re.sub(r'version\s*=\s*"\d+\.\d+\.\d+"', f'version = "{new_version}"', updated_content)

    # 将修改后的内容写回文件
    with open(file_path, 'w', encoding='utf-8') as file:
        file.write(updated_content)
    
    print(f"{file_path} 版本号已更新为: {new_version}")

if __name__ == "__main__":
    # 输入新版本号
    new_version = input("请输入新的版本号: ")

    # 使用正则表达式更新 package.json、Cargo.toml 和 tauri.conf.json 的版本号
    update_version_with_regex('package.json', new_version)
    update_version_with_regex('src-tauri/Cargo.toml', new_version)
    update_version_with_regex('src-tauri/tauri.conf.json', new_version)
    update_version_with_regex('src/pages/About.vue', new_version)
