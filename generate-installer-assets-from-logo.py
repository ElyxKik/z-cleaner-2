#!/usr/bin/env python3
"""
Script pour générer les ressources graphiques de l'installeur Z-Cleaner
à partir du logo PNG fourni.

Génère :
- icon.ico (256x256)
- wizard-image.bmp (164x314)
- wizard-small-image.bmp (55x55)
"""

from PIL import Image, ImageDraw
import os

# Chemin du logo source
LOGO_PATH = "ChatGPT Image 3 sept. 2025, 20_01_25.png"

# Créer le dossier installer s'il n'existe pas
os.makedirs("installer", exist_ok=True)

# Couleurs
BLUE = (63, 127, 255)  # Bleu principal (même que le logo)
DARK_BLUE = (41, 98, 255)  # Bleu foncé
WHITE = (255, 255, 255)

def load_and_resize_logo(size):
    """Charge le logo et le redimensionne"""
    try:
        img = Image.open(LOGO_PATH)
        # Convertir en RGBA si nécessaire
        if img.mode != 'RGBA':
            img = img.convert('RGBA')
        # Redimensionner en gardant les proportions
        img.thumbnail((size, size), Image.Resampling.LANCZOS)
        return img
    except Exception as e:
        print(f"❌ Erreur lors du chargement du logo: {e}")
        raise

def create_icon():
    """Crée l'icône 256x256"""
    print("Création de icon.ico (256x256)...")
    
    # Créer l'image avec fond bleu
    img = Image.new('RGB', (256, 256), BLUE)
    
    # Charger et redimensionner le logo
    logo = load_and_resize_logo(220)
    
    # Centrer le logo
    logo_x = (256 - logo.width) // 2
    logo_y = (256 - logo.height) // 2
    
    # Coller le logo
    img.paste(logo, (logo_x, logo_y), logo)
    
    # Sauvegarder en ICO
    img.save("installer/icon.ico", format='ICO', sizes=[(256, 256)])
    print("✓ icon.ico créé (256x256)")

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
    
    # Charger et redimensionner le logo
    logo = load_and_resize_logo(140)
    
    # Centrer le logo verticalement
    logo_x = (164 - logo.width) // 2
    logo_y = (314 - logo.height) // 2 - 40
    
    # Coller le logo
    img.paste(logo, (logo_x, logo_y), logo)
    
    # Sauvegarder en BMP
    img.save("installer/wizard-image.bmp", format='BMP')
    print("✓ wizard-image.bmp créé (164x314)")

def create_wizard_small_image():
    """Crée la petite image du wizard (55x55)"""
    print("Création de wizard-small-image.bmp (55x55)...")
    
    # Créer l'image avec fond bleu
    img = Image.new('RGB', (55, 55), BLUE)
    
    # Charger et redimensionner le logo
    logo = load_and_resize_logo(50)
    
    # Centrer le logo
    logo_x = (55 - logo.width) // 2
    logo_y = (55 - logo.height) // 2
    
    # Coller le logo
    img.paste(logo, (logo_x, logo_y), logo)
    
    # Sauvegarder en BMP
    img.save("installer/wizard-small-image.bmp", format='BMP')
    print("✓ wizard-small-image.bmp créé (55x55)")

def main():
    print("=" * 60)
    print("Génération des ressources graphiques Z-Cleaner")
    print("=" * 60)
    print()
    
    # Vérifier que le logo existe
    if not os.path.exists(LOGO_PATH):
        print(f"❌ Erreur: Le fichier logo '{LOGO_PATH}' n'existe pas")
        print()
        print("Assurez-vous que le fichier est dans le dossier racine du projet:")
        print(f"  {os.path.abspath(LOGO_PATH)}")
        return False
    
    print(f"✓ Logo trouvé: {LOGO_PATH}")
    print()
    
    try:
        create_icon()
        create_wizard_image()
        create_wizard_small_image()
        
        print()
        print("=" * 60)
        print("✓ Toutes les ressources ont été créées avec succès !")
        print("=" * 60)
        print()
        print("Fichiers créés :")
        print("  ✓ installer/icon.ico (256x256)")
        print("  ✓ installer/wizard-image.bmp (164x314)")
        print("  ✓ installer/wizard-small-image.bmp (55x55)")
        print()
        print("Ces fichiers sont prêts pour l'installeur Inno Setup.")
        print()
        return True
        
    except Exception as e:
        print()
        print("=" * 60)
        print(f"❌ Erreur lors de la génération: {e}")
        print("=" * 60)
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = main()
    exit(0 if success else 1)
