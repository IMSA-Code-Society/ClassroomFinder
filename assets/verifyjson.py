import json

# Load the original JSON data
with open('fixed.json', 'r') as f:
    nodes = json.load(f)

# --- Check 1: Unique IDs ---
ids_seen = set()
duplicates = []

for node in nodes:
    node_id = node['id']
    if node_id in ids_seen:
        duplicates.append(node_id)
    else:
        ids_seen.add(node_id)

if duplicates:
    print("❌ Duplicate IDs found:", duplicates)
else:
    print("✅ All node IDs are unique.")

# --- Check 2: Sequential IDs ---
sorted_ids = sorted(ids_seen)
expected_ids = list(range(sorted_ids[0], sorted_ids[0] + len(sorted_ids)))
missing_ids = sorted(set(expected_ids) - ids_seen)

if missing_ids:
    print("❌ Non-sequential IDs. Missing IDs:", missing_ids)

    # Create mapping from old ID to new sequential ID
    old_to_new_id = {
        old_id: new_id for new_id, old_id in enumerate(sorted_ids, start=sorted_ids[0])
    }

    # Apply the mapping to fix the IDs
    fixed_nodes = []
    for node in nodes:
        new_id = old_to_new_id[node['id']]
        fixed_neighbors = [
            [old_to_new_id[neighbor_id], distance]
            for neighbor_id, distance in node.get('neighbor_nodes', [])
        ]
        fixed_nodes.append({
            "x": node['x'],
            "y": node['y'],
            "name": node.get('name', ''),
            "id": new_id,
            "neighbor_nodes": fixed_neighbors
        })

    with open('fixed.json', 'w') as f_out:
        json.dump(fixed_nodes, f_out, indent=4)

    print(f"✅ Missing IDs fixed and written to 'fixed.json' with remapped IDs.")
    nodes = fixed_nodes  # Continue checks on updated data
    ids_seen = set(node['id'] for node in nodes)

else:
    print("✅ All IDs are sequential with no gaps.")

# --- Check 3: Bidirectional Neighbor Relations ---
id_to_node = {node['id']: node for node in nodes}
inconsistencies = []

for node in nodes:
    node_id = node['id']
    neighbors = node.get('neighbor_nodes', [])

    for neighbor_entry in neighbors:
        neighbor_id = neighbor_entry[0]

        if neighbor_id not in id_to_node:
            inconsistencies.append(
                f"Node {node_id} references nonexistent neighbor {neighbor_id}")
            continue

        neighbor_node = id_to_node[neighbor_id]
        reverse_neighbors = [n[0] for n in neighbor_node.get('neighbor_nodes', [])]

        if node_id not in reverse_neighbors:
            inconsistencies.append(
                f"Node {node_id} has neighbor {neighbor_id}, "
                f"but {neighbor_id} does not reference {node_id} back")

if inconsistencies:
    print("\n❌ Bidirectional neighbor inconsistencies found:")
    for issue in inconsistencies:
        print(" -", issue)
else:
    print("✅ All neighbor relations are bidirectional and valid.")
