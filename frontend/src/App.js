import React, { useState, useEffect } from 'react';
import { ChakraProvider, Box, Heading, Input, Button, VStack, HStack, Text, Checkbox, Select } from '@chakra-ui/react';
import axios from 'axios';

const backendUrl = "http://localhost:8000";  // Adjust based on Rust backend URL

function App() {
  const [todos, setTodos] = useState([]);
  const [newTodo, setNewTodo] = useState('');
  const [filter, setFilter] = useState('all');
  const [editIndex, setEditIndex] = useState(null);
  const [editTodo, setEditTodo] = useState('');

  useEffect(() => {
    fetchTodos();
  }, []);

  const fetchTodos = async () => {
    try {
      const response = await axios.get(`${backendUrl}/list`);
      setTodos(response.data);
    } catch (error) {
      console.error('Error fetching todos:', error);
    }
  };

  const addTodo = async () => {
    if (newTodo.trim()) {
      try {
        const todo = { task: newTodo, is_complete: false };
        await axios.post(`${backendUrl}/add`, todo);
        setTodos([...todos, todo]);
        setNewTodo('');
      } catch (error) {
        console.error('Error adding todo:', error);
      }
    }
  };

  const toggleComplete = async (index) => {
    const newTodos = [...todos];
    newTodos[index].is_complete = !newTodos[index].is_complete;
    setTodos(newTodos);
  };

  const deleteTodo = async (index) => {
    setTodos(todos.filter((_, i) => i !== index));
  };

  const handleEdit = (index) => {
    setEditIndex(index);
    setEditTodo(todos[index].task);
  };

  const saveEdit = async (index) => {
    const newTodos = [...todos];
    newTodos[index].task = editTodo;
    setTodos(newTodos);
    setEditIndex(null);
    setEditTodo('');
  };

  const filterTodos = () => {
    if (filter === 'completed') return todos.filter(todo => todo.is_complete);
    if (filter === 'active') return todos.filter(todo => !todo.is_complete);
    return todos;
  };

  return (
    <ChakraProvider>
      <Box w="100%" p={5}>
        <VStack spacing={5}>
          <Heading mb={4}>Todo List</Heading>
          <HStack>
            <Input
              value={newTodo}
              onChange={(e) => setNewTodo(e.target.value)}
              placeholder="Add new task"
            />
            <Button colorScheme="teal" onClick={addTodo}>
              Add Task
            </Button>
          </HStack>

          <Select value={filter} onChange={(e) => setFilter(e.target.value)}>
            <option value="all">All</option>
            <option value="completed">Completed</option>
            <option value="active">Active</option>
          </Select>

          {filterTodos().map((todo, index) => (
            <HStack key={index} justify="space-between" w="100%" p={2} borderBottom="1px" borderColor="gray.200">
              <Checkbox isChecked={todo.is_complete} onChange={() => toggleComplete(index)}>
                {editIndex === index ? (
                  <Input value={editTodo} onChange={(e) => setEditTodo(e.target.value)} />
                ) : (
                  <Text as={todo.is_complete ? 'del' : ''}>{todo.task}</Text>
                )}
              </Checkbox>
              {editIndex === index ? (
                <Button size="sm" colorScheme="blue" onClick={() => saveEdit(index)}>Save</Button>
              ) : (
                <Button size="sm" colorScheme="gray" onClick={() => handleEdit(index)}>Edit</Button>
              )}
              <Button size="sm" colorScheme="red" onClick={() => deleteTodo(index)}>Delete</Button>
            </HStack>
          ))}
        </VStack>
      </Box>
    </ChakraProvider>
  );
}

export default App;
