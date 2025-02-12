#include <iostream>

#include "demo/include/blobstore.h"
#include "demo/src/main.rs.h"

int main() {
  auto client = org::blobstore::new_blobstore_client();

  // Upload a blob.
  std::string str("hello world");
  std::vector<uint8_t> vec(str.begin(), str.end());
  ::rust::Vec<uint8_t> rvec{};
  std::copy(vec.begin(), vec.end(), std::back_inserter(rvec));
  ::rust::Box<org::blobstore::MultiBuf> buf =
      org::blobstore::create_multibuf(rvec);
  auto blobid = client->put(*buf);

  // Add a tag.
  client->tag(blobid, "rust");

  // Read back the tags.
  auto metadata = client->metadata(blobid);
  std::cout << "tags = " << metadata.tags[0] << std::endl;
}
