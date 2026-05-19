import os
import re
from pathlib import Path

def sanitize_id(filename):
    """
    Преобразует имя файла в валидный Fluent ID.
    Разрешены только a-z, A-Z, 0-9, -, _
    ID должен начинаться с буквы.
    """
    # Берем имя файла без расширения
    name = Path(filename).stem
    # Заменяем все недопустимые символы на дефис
    sanitized = re.sub(r'[^a-zA-Z0-9_-]', '-', name)
    
    # Если после очистки строка пустая или начинается не с буквы, добавляем префикс
    if not sanitized or not sanitized[0].isalpha():
        sanitized = 'file-' + sanitized
        
    return sanitized

def escape_fluent_text(text):
    """
    Экранирует фигурные скобки, так как в Fluent они зарезервированы 
    для подстановок (placeables).
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
    
    # Открываем файл для записи результата
    with open(output_file, 'w', encoding='utf-8') as out_ftl:
        
        # Перебираем все файлы в директории
        for file_path in directory.iterdir():
            if not file_path.is_file():
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
            except UnicodeDecodeError:
                # Пропускаем бинарные файлы (картинки и т.д.)
                print(f"Пропущен нетекстовый файл: {file_path.name}")
                continue
                
            if not content.strip():
                continue # Пропускаем пустые файлы
                
            lines = content.splitlines()
            
            # 1. Значение: первая строка файла
            first_line = escape_fluent_text(lines[0].strip())
            
            # 2. ИД: имя файла
            msg_id = sanitize_id(file_path.name)
            
            # 3. Атрибут .markdown: весь файл
            # В Fluent многострочные значения должны быть с отступом (например, 8 пробелов)
            markdown_attr_lines = []
            for line in lines:
                escaped_line = escape_fluent_text(line)
                if escaped_line.strip():
                    markdown_attr_lines.append(f"        {escaped_line}")
                else:
                    # Пустые строки можно оставлять без отступа
                    markdown_attr_lines.append("") 
            
            markdown_attr = "\n".join(markdown_attr_lines)
            
            # Записываем сформированный блок в FTL файл
            out_ftl.write(f"{msg_id} = {first_line}\n")
            out_ftl.write(f"    .markdown =\n{markdown_attr}\n\n")
            
    print(f"Готово! Файл сохранен как {output_file}")

# === ИСПОЛЬЗОВАНИЕ ===
# Укажите путь к папке с вашими файлами и имя итогового файла
SOURCE_DIR = './book/markdown/en'  # Замените на путь к вашей директории
OUTPUT_FTL = 'output.ftl'

# Создаем тестовую директорию, если её нет (для примера)
if not os.path.exists(SOURCE_DIR):
    os.makedirs(SOURCE_DIR)
    print(f"Пожалуйста, положите ваши файлы в папку '{SOURCE_DIR}' и запустите скрипт снова.")
else:
    create_ftl_from_dir(SOURCE_DIR, OUTPUT_FTL)