import os
import re
from pathlib import Path

def sanitize_id(name):
    """
    Преобразует строку в валидный Fluent ID.
    Разрешены только a-z, A-Z, 0-9, -, _
    ID должен начинаться с буквы.
    """
    # Заменяем все недопустимые символы на дефис
    sanitized = re.sub(r'[^a-zA-Z0-9_-]', '-', name)
    
    # Убираем лишние дефисы подряд (для красоты)
    sanitized = re.sub(r'-+', '-', sanitized).strip('-')
    
    # Если после очистки строка пустая или начинается не с буквы, добавляем префикс
    if not sanitized or not sanitized[0].isalpha():
        sanitized = 'file-' + sanitized
        
    return sanitized

def escape_fluent_text(text):
    """
    Экранирует фигурные скобки для Fluent.
    """
    res = []
    for char in text:
        if char == '{':
            res.append('{"{"}')
        elif char == '}':
            res.append('{"}"}')
        else:
            res.append(char)
    return "".join(res)

def create_ftl_from_dir(directory_path, output_file):
    directory = Path(directory_path)
    
    with open(output_file, 'w', encoding='utf-8') as out_ftl:
        
        # rglob('*') рекурсивно обходит все файлы и папки
        for file_path in directory.rglob('*'):
            # Пропускаем папки
            if not file_path.is_file():
                continue
                
            # ПРОПУСКАЕМ index.md (независимо от регистра, например Index.md тоже пропустится)
            if file_path.name.lower() == 'index.md':
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
            except UnicodeDecodeError:
                print(f"Пропущен нетекстовый файл: {file_path}")
                continue
                
            if not content.strip():
                continue # Пропускаем пустые файлы
                
            lines = content.splitlines()
            
            # 1. Значение: первая строка файла
            first_line = escape_fluent_text(lines[0].strip())
            
            # 2. ИД: формируем из относительного пути, чтобы избежать дублей
            # Например: folder/subfolder/file.md -> folder-subfolder-file
            rel_path = file_path.relative_to(directory)
            parts = list(rel_path.parts)
            parts[-1] = file_path.stem # Убираем расширение у самого файла
            id_base = "-".join(parts)
            
            msg_id = sanitize_id(id_base)
            
            # 3. Атрибут .markdown: весь файл
            markdown_attr_lines = []
            for line in lines:
                escaped_line = escape_fluent_text(line)
                if escaped_line.strip():
                    markdown_attr_lines.append(f"        {escaped_line}")
                else:
                    markdown_attr_lines.append("") 
            
            markdown_attr = "\n".join(markdown_attr_lines)
            
            # Записываем в FTL
            out_ftl.write(f"{msg_id} = {first_line}\n")
            out_ftl.write(f"    .markdown =\n{markdown_attr}\n\n")
            
    print(f"Готово! Файл сохранен как {output_file}")

# === ИСПОЛЬЗОВАНИЕ ===
SOURCE_DIR = './book/markdown/en'  # Путь к корневой директории
OUTPUT_FTL = 'output.ftl'

if not os.path.exists(SOURCE_DIR):
    os.makedirs(SOURCE_DIR)
    print(f"Создана папка '{SOURCE_DIR}'. Положите туда файлы и запустите скрипт снова.")
else:
    create_ftl_from_dir(SOURCE_DIR, OUTPUT_FTL)