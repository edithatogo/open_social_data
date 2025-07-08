import requests
import json

def get_abs_dataflows():
    """
    Fetch and display a list of all available dataflows (datasets) from the Australian Bureau of Statistics (ABS) SDMX-JSON API.
    
    Sends a request to the ABS dataflow endpoint, parses the response, and prints each dataflow's ID, agency, version, and English name. Handles network and parsing errors with diagnostic output.
    """
    # Endpoint to get all dataflows from ABS agency, with referencepartial detail.
    # referencepartial gives us enough detail including the names and IDs.
    # Note: The API documentation suggests 'format=sdmx-json' for general SDMX-JSON,
    # but for structure messages like dataflow listings or DSDs,
    # the Accept header 'application/vnd.sdmx.structure+json' is more specific.
    # The format query parameter might be redundant or an alternative way if Accept header is not used.
    dataflow_list_url = "https://api.abs.gov.au/dataflow/ABS/all?detail=referencepartial" # Removed format=sdmx-json as Accept header is used

    headers = {
        'Accept': 'application/vnd.sdmx.structure+json;version=2.0.0' # Using v2.0.0 for structure
    }

    try:
        print(f"Fetching dataflows from URL: {dataflow_list_url}")
        response = requests.get(dataflow_list_url, headers=headers, timeout=60)
        response.raise_for_status()  # Raises an HTTPError for bad responses (4XX or 5XX)

        data = response.json()

        print("Successfully fetched dataflows. Processing...")

        # SDMX-JSON v2.0.0 structure for dataflow list
        if 'data' in data and 'dataflows' in data['data']:
            dataflows = data['data']['dataflows']
            print(f"\nFound {len(dataflows)} dataflows:\n")
            for df_item in dataflows:
                df_id = df_item.get('id', 'N/A')
                df_agency = df_item.get('agencyID', 'N/A')
                df_version = df_item.get('version', 'N/A')

                df_name = "Unknown Name"
                # Name is a list of localized names in v2.0.0
                if 'name' in df_item and isinstance(df_item['name'], list):
                    for name_obj in df_item['name']:
                        if name_obj.get('locale') == 'en': # 'locale' for v2.0.0, 'lang' for v1.0.0
                            df_name = name_obj.get('value', 'No name found')
                            break

                print(f"ID: {df_id}, Agency: {df_agency}, Version: {df_version}, Name: {df_name}")
        else:
            print("No 'data.dataflows' key found in the response. Full response data (first 500 chars):")
            print(json.dumps(data, indent=2)[:500] + "...")

    except requests.exceptions.HTTPError as http_err:
        print(f"HTTP error occurred: {http_err}")
        print(f"Response content: {response.content.decode(errors='ignore') if response.content else 'No content'}")
    except requests.exceptions.ConnectionError as conn_err:
        print(f"Connection error occurred: {conn_err}")
    except requests.exceptions.Timeout as timeout_err:
        print(f"Timeout error occurred: {timeout_err}")
    except requests.exceptions.RequestException as req_err:
        print(f"An error occurred: {req_err}")
    except json.JSONDecodeError:
        print("Failed to decode JSON response.")
        print(f"Response content: {response.text}")

def get_abs_datastructure(dataflow_id):
    """
    Retrieve the Data Structure Definition (DSD) for a specified ABS dataflow ID using the SDMX-JSON API.
    
    Parameters:
        dataflow_id (str): The identifier of the ABS dataflow (e.g., "QBIS").
    
    Returns:
        dict | None: The parsed DSD JSON data if retrieval is successful; otherwise, None if an error occurs.
    """
    # URL pattern for DSD: https://api.abs.gov.au/datastructure/ABS/{dataflow_id}/latest?detail=full&references=children
    # Using "ABS" as agency and "latest" as version.
    url = f"https://api.abs.gov.au/datastructure/ABS/{dataflow_id}/latest?detail=full&references=children"

    headers = {
        'Accept': 'application/vnd.sdmx.structure+json;version=2.0.0' # Using SDMX-JSON Structure v2.0.0
    }

    try:
        print(f"Fetching DSD from URL: {url}")
        response = requests.get(url, headers=headers, timeout=60)
        response.raise_for_status()

        data = response.json()
        print(f"Successfully fetched DSD for {dataflow_id}.")
        return data

    except requests.exceptions.HTTPError as http_err:
        print(f"HTTP error occurred while fetching DSD for {dataflow_id}: {http_err}")
        print(f"Response content: {response.content.decode(errors='ignore') if response.content else 'No content'}")
    except requests.exceptions.ConnectionError as conn_err:
        print(f"Connection error occurred while fetching DSD for {dataflow_id}: {conn_err}")
    except requests.exceptions.Timeout as timeout_err:
        print(f"Timeout error occurred while fetching DSD for {dataflow_id}: {timeout_err}")
    except requests.exceptions.RequestException as req_err:
        print(f"An error occurred while fetching DSD for {dataflow_id}: {req_err}")
    except json.JSONDecodeError:
        print(f"Failed to decode JSON response for DSD {dataflow_id}.")
        print(f"Response content: {response.text}")
    return None

if __name__ == "__main__":
    # Example usage (can be commented out or changed as needed)
    # get_abs_dataflows()

    # Fetch DSD for QBIS
    dsd_data = get_abs_datastructure("QBIS")
    if dsd_data:
        print("\n\n--- Data Structure Definition (DSD) for QBIS ---")
        # print(json.dumps(dsd_data, indent=2)) # Print full DSD if needed for debugging

        # SDMX-JSON v2.0.0 structure for DSDs is typically under `data.structures[0]`
        # The DSD itself is one of the structures in the 'structures' list.
        # We need to find the one that is a DataStructureDefinition.
        dsd_object = None
        if 'data' in dsd_data and 'structures' in dsd_data['data']:
            for structure_item in dsd_data['data']['structures']:
                if structure_item.get('@type') == 'DataStructure':
                    dsd_object = structure_item
                    break

        if dsd_object:
            dsd_name_obj = dsd_object.get('name', [{}])[0] # Name is usually a list with language variants
            dsd_name = "Unknown DataStructure"
            # Iterate through names to find English one
            for name_entry in dsd_object.get('name', []):
                if name_entry.get('locale') == 'en':
                    dsd_name = name_entry.get('value', 'Unknown DataStructure')
                    break

            print(f"\nFound DSD: {dsd_name} (ID: {dsd_object.get('id')})")

            components = dsd_object.get('components', {})
            dimensions = components.get('dimensions', [])
            attributes = components.get('attributes', [])
            # In SDMX v2.0, the primary measure is usually listed under 'measureDimensions' or as a specific 'measure' component.
            # For simplicity, let's look for a measure component first.
            primary_measure_component = components.get('measure', [{}])[0] # Assuming it's a list, take first

            print("\nDimensions (referencing concepts):")
            for dim_ref in dimensions:
                dim_id = dim_ref.get('id', 'N/A')
                concept_urn = dim_ref.get('conceptIdentity') # URN of the concept
                # Extract a readable name from the URN if possible
                dim_name_from_urn = concept_urn.split('.')[-1] if concept_urn else dim_id

                print(f"  ID: {dim_id}, Concept: {dim_name_from_urn} (URN: {concept_urn})")

                codelist_ref = dim_ref.get('localRepresentation', {}).get('enumeration') # URN of the codelist
                if codelist_ref:
                    print(f"    Codelist URN: {codelist_ref}")


            print("\nAttributes (referencing concepts):")
            for attr_ref in attributes:
                attr_id = attr_ref.get('id', 'N/A')
                concept_urn = attr_ref.get('conceptIdentity')
                attr_name_from_urn = concept_urn.split('.')[-1] if concept_urn else attr_id
                assignment_status = attr_ref.get('assignmentStatus', 'N/A')
                relationship = attr_ref.get('relationship', {}).get('primaryMeasure') # Check if related to primary measure

                print(f"  ID: {attr_id}, Concept: {attr_name_from_urn} (URN: {concept_urn}), Status: {assignment_status}, RelatedToPrimaryMeasure: {bool(relationship)}")
                codelist_ref = attr_ref.get('localRepresentation', {}).get('enumeration')
                if codelist_ref:
                     print(f"    Codelist URN: {codelist_ref}")


            print("\nPrimary Measure (referencing concept):")
            if primary_measure_component and primary_measure_component.get('id'):
                measure_id = primary_measure_component.get('id', 'N/A')
                concept_urn = primary_measure_component.get('conceptIdentity')
                measure_name_from_urn = concept_urn.split('.')[-1] if concept_urn else measure_id
                print(f"  ID: {measure_id}, Concept: {measure_name_from_urn} (URN: {concept_urn})")
            else:
                print("  Primary Measure component not clearly identified in DSD components.measure.")
        else:
            print("Could not find DataStructure type object in the DSD response's data.structures.")
            print("Full DSD response (first 500 chars):")
            print(json.dumps(dsd_data, indent=2)[:500] + "...")
