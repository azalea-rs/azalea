#!/usr/bin/env python3

import os
import json
from zipfile import ZipFile
from lib.download import get_client_jar
from lib.utils import get_dir_location
from lib.code.version import get_version_id


def extract_assets(version_id: str):
    """Extract the full assets folder from the client jar"""
    get_client_jar(version_id)
    
    client_jar_path = get_dir_location(f"__cache__/client-{version_id}.jar")
    assets_output_dir = get_dir_location(f"../azalea-graphics/assets")
    
    print(f"Extracting assets from {client_jar_path} to {assets_output_dir}")
    
    # Create output directory if it doesn't exist
    os.makedirs(assets_output_dir, exist_ok=True)
    
    with ZipFile(client_jar_path, 'r') as jar:
        # Get all entries that start with "assets/"
        asset_entries = [name for name in jar.namelist() if name.startswith("assets/")]
        
        print(f"Found {len(asset_entries)} asset files")
        
        for entry in asset_entries:
            # Skip directories
            if entry.endswith('/'):
                continue
                
            # Remove "assets/" prefix to avoid nested structure
            relative_path = entry[len("assets/"):]
            output_path = os.path.join(assets_output_dir, relative_path)
            
            # Create parent directories if they don't exist
            os.makedirs(os.path.dirname(output_path), exist_ok=True)
            
            # Extract the file
            with jar.open(entry) as source:
                with open(output_path, 'wb') as target:
                    target.write(source.read())

    
    print(f"Asset extraction complete! Assets saved to: {assets_output_dir}")


if __name__ == "__main__":
    extract_assets(get_version_id())
