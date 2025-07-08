import requests
import json

BASE_URL = "https://myhospitalsapi.aihw.gov.au/api/v1"

def get_aihw_measure_categories():
    """
    Fetches the list of measure categories from the AIHW MyHospitals API.
    """
    url = f"{BASE_URL}/measure-categories"
    headers = {
        'Accept': 'application/json',
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
    }
    print(f"Fetching measure categories from: {url}")
    try:
        response = requests.get(url, headers=headers, timeout=30)
        response.raise_for_status()  # Raises an HTTPError for bad responses (4XX or 5XX)

        data = response.json()
        print("Successfully fetched measure categories.")
        return data

    except requests.exceptions.HTTPError as http_err:
        print(f"HTTP error occurred: {http_err}")
        print(f"Response content: {response.content.decode(errors='ignore') if response.content else 'No content'}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred: {e}")
    except json.JSONDecodeError:
        print(f"Failed to decode JSON response.")
        print(f"Response content: {response.text}")
    return None

if __name__ == "__main__":
    print("--- AIHW MyHospitals API Fetcher ---")

    measure_categories = get_aihw_measure_categories()

    if measure_categories and 'result' in measure_categories:
        actual_categories = measure_categories['result']
        version_info = measure_categories.get('version_information', {})

        print(f"\nFound {len(actual_categories)} measure categories (API Version: {version_info.get('api_version', 'N/A')}, Data Version: {version_info.get('data_version', 'N/A')}):\n")
        for category in actual_categories:
            print(f"  Code: {category.get('measure_category_code')}, Name: {category.get('measure_category_name')}")
    elif measure_categories:
        print("\nReceived data, but 'result' key is missing. Raw data:")
        print(json.dumps(measure_categories, indent=2))
    else:
        print("No measure categories data received.")
