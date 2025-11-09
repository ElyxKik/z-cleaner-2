#!/usr/bin/env python3
"""
Script pour générer les ressources graphiques de l'installeur Z-Cleaner
Génère :
- icon.ico (256x256)
- wizard-image.bmp (164x314)
- wizard-small-image.bmp (55x55)
"""

from PIL import Image, ImageDraw, ImageFont
import os

# Créer le dossier installer/assets s'il n'existe pas
os.makedirs("installer/assets", exist_ok=True)

# Couleurs
BLUE = (63, 127, 255)  # Bleu principal
DARK_BLUE = (41, 98, 255)  # Bleu foncé
WHITE = (255, 255, 255)
LIGHT_GRAY = (240, 240, 240)
DARK_GRAY = (50, 50, 50)

def create_z_logo(size):
    """Crée le logo Z-Cleaner"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Calculer les dimensions
    margin = size // 8
    stroke_width = max(1, size // 20)
    
    # Dessiner le Z blanc
    z_x1 = margin
    z_y1 = margin
    z_x2 = size - margin
    z_y2 = size - margin
    
    # Haut du Z
    draw.rectangle(
        [z_x1, z_y1, z_x2, z_y1 + stroke_width * 3],
        fill=WHITE
    )
    
    # Diagonale du Z
    points = [
        (z_x2, z_y1 + stroke_width * 3),
        (z_x1, z_y2 - stroke_width * 3)
    ]
    draw.line(points, fill=WHITE, width=stroke_width * 3)
    
    # Bas du Z
    draw.rectangle(
        [z_x1, z_y2 - stroke_width * 3, z_x2, z_y2],
        fill=WHITE
    )
    
    # Dessiner le balai
    brush_x = z_x2 + margin // 2
    brush_y = z_y1 + size // 4
    
    # Manche du balai
    draw.rectangle(
        [brush_x - stroke_width, brush_y, brush_x + stroke_width, brush_y + size // 3],
        fill=WHITE
    )
    
    # Poils du balai
    brush_width = size // 6
    for i in range(5):
        y_offset = brush_y + size // 4 + i * stroke_width
        draw.ellipse(
            [brush_x - brush_width // 2, y_offset, brush_x + brush_width // 2, y_offset + stroke_width * 2],
            fill=WHITE
        )
    
    return img

def create_icon():
    """Crée l'icône 256x256"""
    print("Création de icon.ico (256x256)...")
    
    # Créer l'image avec fond bleu
    img = Image.new('RGB', (256, 256), BLUE)
    
    # Ajouter le logo Z
    logo = create_z_logo(200)
    # Centrer le logo
    logo_x = (256 - 200) // 2
    logo_y = (256 - 200) // 2
    img.paste(logo, (logo_x, logo_y), logo)
    
    # Sauvegarder en ICO
    img.save("installer/assets/icon.ico", format='ICO', sizes=[(256, 256)])
    print("✓ icon.ico créé")

def create_wizard_image():
    """Crée l'image du wizard (164x314)"""
    print("Création de wizard-image.bmp (164x314)...")
    
    # Créer l'image avec dégradé bleu
    img = Image.new('RGB', (164, 314), BLUE)
    draw = ImageDraw.Draw(img)
    
    # Ajouter un dégradé (du bleu clair au bleu foncé)
    for y in range(314):
        ratio = y / 314
        r = int(63 + (41 - 63) * ratio)
        g = int(127 + (98 - 127) * ratio)
        b = int(255)
        draw.line([(0, y), (164, y)], fill=(r, g, b))
    
    # Ajouter le logo Z-Cleaner
    logo = create_z_logo(120)
    logo_x = (164 - 120) // 2
    logo_y = (314 - 120) // 2 - 30
    img.paste(logo, (logo_x, logo_y), logo)
    
    # Ajouter du texte "Z-Cleaner"
    try:
        # Essayer d'utiliser une police système
        font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 24)
    except:
        # Utiliser la police par défaut
        font = ImageFont.load_default()
    
    text = "Z-Cleaner"
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_x = (164 - text_width) // 2
    text_y = logo_y + 120 + 20
    
    draw.text((text_x, text_y), text, fill=WHITE, font=font)
    
    # Sauvegarder en BMP
    img.save("installer/assets/wizard-image.bmp", format='BMP')
    print("✓ wizard-image.bmp créé")

def create_wizard_small_image():
    """Crée la petite image du wizard (55x55)"""
    print("Création de wizard-small-image.bmp (55x55)...")
    
    # Créer l'image avec fond bleu
    img = Image.new('RGB', (55, 55), BLUE)
    
    # Ajouter le logo Z
    logo = create_z_logo(45)
    logo_x = (55 - 45) // 2
    logo_y = (55 - 45) // 2
    img.paste(logo, (logo_x, logo_y), logo)
    
    # Sauvegarder en BMP
    img.save("installer/assets/wizard-small-image.bmp", format='BMP')
    print("✓ wizard-small-image.bmp créé")

def main():
    print("=" * 50)
    print("Génération des ressources graphiques Z-Cleaner")
    print("=" * 50)
    print()
    
    try:
        create_icon()
        create_wizard_image()
        create_wizard_small_image()
        
        print()
        print("=" * 50)
        print("✓ Toutes les ressources ont été créées !")
        print("=" * 50)
        print()
        print("Fichiers créés :")
        print("  - installer/assets/icon.ico (256x256)")
        print("  - installer/assets/wizard-image.bmp (164x314)")
        print("  - installer/assets/wizard-small-image.bmp (55x55)")
        print()
        print("Copier ces fichiers vers le dossier 'installer/' :")
        print("  cp installer/assets/icon.ico installer/")
        print("  cp installer/assets/wizard-image.bmp installer/")
        print("  cp installer/assets/wizard-small-image.bmp installer/")
        
    except Exception as e:
        print(f"❌ Erreur : {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()
