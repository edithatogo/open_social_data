import requests
import pandas as pd
import json
import os
import logging
from datetime import datetime

# Configure logging
# Assuming the script is run from the root of the repository or paths are relative to /app
# Correct paths based on the known location of the script:
# /app/datasets/aihw/myhospitals/scripts/fetch_aihw_myhospitals_data.py
LOG_DIR = "/app/datasets/aihw/myhospitals/logs/"
DATA_DIR = "/app/datasets/aihw/myhospitals/data/"

# Assuming directories are created by an earlier setup step (e.g., bash mkdir -p)
# os.makedirs(LOG_DIR, exist_ok=True)
# os.makedirs(DATA_DIR, exist_ok=True)

LOG_FILE = os.path.join(LOG_DIR, "myhospitals_processing.log")

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler(LOG_FILE),
        logging.StreamHandler()
    ]
)

BASE_URL = "https://myhospitalsapi.aihw.gov.au/api/v1"
USER_AGENT = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'

# DATA_DIR is defined above with an absolute path. This block is redundant.
# # Define output directory for data
# DATA_DIR = "../data/" # Relative to this script's location
# os.makedirs(DATA_DIR, exist_ok=True)

def fetch_flat_formatted_data(measure_category_code, skip=0, top=100):
    """
    Fetches flat formatted data for a given measure category code with pagination.
    """
    url = f"{BASE_URL}/flat-formatted-data-extract/{measure_category_code}"
    headers = {
        'Accept': 'application/json',
        'User-Agent': USER_AGENT
    }
    params = {
        'skip': skip,
        'top': top
    }

    logging.info(f"Fetching data from: {url} with params: {params}")
    try:
        response = requests.get(url, headers=headers, params=params, timeout=60)
        response.raise_for_status()

        data = response.json()
        logging.info(f"Successfully fetched data for {measure_category_code}, skip={skip}, top={top}.")
        return data

    except requests.exceptions.HTTPError as http_err:
        logging.error(f"HTTP error occurred: {http_err}")
        logging.error(f"Response content: {response.content.decode(errors='ignore') if response.content else 'No content'}")
    except requests.exceptions.RequestException as e:
        logging.error(f"An error occurred: {e}")
    except json.JSONDecodeError:
        logging.error(f"Failed to decode JSON response.")
        logging.error(f"Response content: {response.text}")
    return None

def process_and_save_data(measure_category_code, filename_prefix="aihw_myhospitals"):
    """
    Fetches all data for a measure category using pagination,
    processes it into a Pandas DataFrame, and saves it as a Parquet file.
    """
    all_data_items = []
    skip = 0
    top = 1000  # Max allowed by API according to Swagger for flat data extracts
    page_num = 1

    while True:
        logging.info(f"Fetching page {page_num} (skip={skip}, top={top}) for {measure_category_code}...")
        api_response = fetch_flat_formatted_data(measure_category_code, skip=skip, top=top)

        if api_response and 'result' in api_response and \
           'data' in api_response['result'] and \
           isinstance(api_response['result']['data'], list):

            result_content = api_response['result']
            current_page_items = result_content['data']

            if not current_page_items:
                logging.info("No more items found in 'result.data' on this page. Assuming end of data.")
                break
            all_data_items.extend(current_page_items)

            pagination_info = result_content.get('pagination', {})
            results_returned = pagination_info.get('results_returned', len(current_page_items))
            # total_results_available = pagination_info.get('total_results_available', 0) # This can be logged if needed

            logging.info(f"Page {page_num}: Fetched {results_returned} items. Total items so far: {len(all_data_items)}.")

            if len(current_page_items) < top:
                logging.info("Fewer items returned than 'top' parameter; assuming this is the last page.")
                break

            skip += top
            page_num += 1

            # Optional safety break for very long runs if needed in other contexts
            # if page_num > 200: # Example: limit to 200 pages if total_results_available is not trusted
            #     logging.warning(f"Reached a hardcoded page limit of 200 for {measure_category_code}. Stopping.")
            #     break

        elif api_response and ('result' not in api_response or 'data' not in api_response.get('result', {})):
            logging.info(f"Page {page_num}: Response structure is not as expected (missing 'result' or 'result.data'). Full response: {json.dumps(api_response, indent=2)}")
            break
        else: # api_response is None or some other issue
            logging.error(f"Failed to fetch page {page_num} or API response was None. Stopping.")
            return False # Indicate failure

    if not all_data_items:
        logging.info(f"No data items found for {measure_category_code} after attempting to fetch all pages.")
        return True # No data is not an error, but nothing to save

    logging.info(f"Total items fetched for {measure_category_code}: {len(all_data_items)}")

    # Convert to DataFrame
    df = pd.DataFrame(all_data_items)
    logging.info(f"Converted to DataFrame with shape: {df.shape}")

    if df.empty:
        logging.info(f"DataFrame is empty for {measure_category_code}. Nothing to save.")
        return True

    # Save to Parquet
    # Generate a timestamped filename to avoid overwriting and keep history if needed
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    file_path = os.path.join(DATA_DIR, f"{filename_prefix}_{measure_category_code}_{timestamp}.parquet")

    try:
        df.to_parquet(file_path, index=False)
        logging.info(f"Successfully saved data to {file_path}")
        return True
    except Exception as e:
        logging.error(f"Error saving DataFrame to Parquet: {e}")
        return False

if __name__ == "__main__":
    logging.info("--- Starting AIHW MyHospitals Data Fetching Script ---")

    # For initial testing, let's fetch a small amount of data for MYH-ED-WAITS
    # We'll call fetch_flat_formatted_data directly for this test.

    test_measure_category = "MYH-ES" # MyHospitals Elective Surgery
    logging.info(f"Performing initial small fetch for category: {test_measure_category}")

    # Fetch only the first few items for inspection
    initial_data_sample = fetch_flat_formatted_data(test_measure_category, skip=0, top=10)

    if initial_data_sample:
        logging.info(f"Raw initial_data_sample content: {json.dumps(initial_data_sample, indent=2)}")
    else:
        logging.info("initial_data_sample is None.")

    if initial_data_sample and 'result' in initial_data_sample and \
       'data' in initial_data_sample['result'] and \
       isinstance(initial_data_sample['result']['data'], list):

        result_content = initial_data_sample['result']
        sample_items = result_content['data']
        pagination = result_content.get('pagination')

        logging.info(f"Sample data (first {len(sample_items)} items) for {test_measure_category}:")
        # Pretty print the first item if available
        if sample_items:
            logging.info(f"First sample item: {json.dumps(sample_items[0], indent=2)}")
        else:
            logging.info("No items in the sample data list.")
        logging.info(f"Pagination info for sample: {pagination}")

        # Example of how to convert this small sample to DataFrame and save (optional for testing)
        # if sample_items:
        #     df_sample = pd.DataFrame(sample_items)
        #     sample_file_path = os.path.join(DATA_DIR, f"sample_{test_measure_category}.parquet")
        #     df_sample.to_parquet(sample_file_path, index=False)
        #     logging.info(f"Saved sample data to {sample_file_path}")

    else:
        logging.warning(f"Could not fetch initial sample for {test_measure_category}.")

    # To run the full processing and save for a category:
    # Comment out the sample fetching above and uncomment below.
    # Be mindful of API rate limits and data size.
    logging.info(f"\n--- Starting full data processing for category: {test_measure_category} ---")
    success = process_and_save_data(test_measure_category, filename_prefix="aihw_myhospitals") # Corrected prefix
    if success:
        logging.info(f"Full processing for {test_measure_category} completed.")
    else:
        logging.error(f"Full processing for {test_measure_category} failed or had issues.")

    logging.info("--- AIHW MyHospitals Data Fetching Script Finished ---")
