/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
public:
    void rec_post (Node* node, vector<int>& arr){
        for (auto &child : node->children){
            rec_post(child, arr);
        }
        arr.push_back (node->val);
    }

    vector<int> postorder(Node* root) {
        std::vector<int> ret;
        if (root != NULL) {
            rec_post(root, ret);
        }
        return ret;
    }
};
