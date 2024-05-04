import { useState } from 'react';

import { PencilIcon, TrashIcon } from '@heroicons/react/outline';
import { CheckIcon, MinusSmIcon } from '@heroicons/react/solid';

import { useStore } from '../store/store';
import { TodoItem } from '../types/TodoItem';
import { TodoList } from '../types/TodoList';
import Button from './Button';
import DeleteListModal from './DeleteListModal';
import Input from './Input';
import RenameListModal from './RenameListModal';

type TodoListProps = {
  todoList: TodoList;
};

export default function TodoListComponent(props: TodoListProps) {
  const { todoList } = props;

  const onRenameTodoList = useStore((store) => store.renameTodoList);
  const onDeleteTodoList = useStore((store) => store.deleteTodoList);
  const onCreateTodoItem = useStore((store) => store.createTodoItem);
  const onDeleteTodoItem = useStore((store) => store.deleteTodoItem);
  const onUpdateTodoItemComplete = useStore(
    (store) => store.updateTodoItemComplete
  );
  const [renameListModalOpen, setRenameListModalOpen] = useState(false);
  const [deleteListModalOpen, setDeleteListModalOpen] = useState(false);
  const [newTodoItemText, setNewTodoItemText] = useState('');

  function handleOnCreateTodoItem(e: React.FormEvent<HTMLFormElement>): void {
    e.preventDefault();
    if (newTodoItemText.length > 0) {
      onCreateTodoItem(todoList.id, newTodoItemText);
      setNewTodoItemText('');
    }
  }

  return (
    <div className="flex flex-col grow px-4 py-2">
      <div className="flex flex-row justify-between">
        <Button
          icon={<PencilIcon className="h-4 w-4" />}
          onClick={() => setRenameListModalOpen(true)}
        />
        <p className="font-medium ml-4 text-center text-2xl">{todoList.name}</p>
        <Button
          icon={<TrashIcon className="h-4 w-4" />}
          onClick={() => setDeleteListModalOpen(true)}
        />
      </div>
      <div className="flex flex-col gap-4 grow my-4 overflow-y-auto">
        {todoList.todos.map((todoItem) => (
          <TodoItemComponent
            onDelete={(todoItemId: number) =>
              onDeleteTodoItem(todoList.id, todoItemId)
            }
            onUpdateComplete={(todoItemId: number, complete: boolean) =>
              onUpdateTodoItemComplete(todoList.id, todoItemId, complete)
            }
            todoItem={todoItem}
          />
        ))}
      </div>
      <form
        className="border border-gray-100 dark:border-slate-700 flex flex-row px-4 py-4 rounded-md shadow-sm"
        onSubmit={(e) => handleOnCreateTodoItem(e)}
      >
        <Input
          onChange={(v: string) => setNewTodoItemText(v)}
          placeholder="Create a new to-do..."
          value={newTodoItemText}
        />
        <Button
          className="ml-2"
          icon={<CheckIcon className="h-4 w-4" />}
          type="submit"
          variant="primary"
        />
      </form>
      <RenameListModal
        open={renameListModalOpen}
        onClose={() => setRenameListModalOpen(false)}
        onRename={(newName: string) => onRenameTodoList(todoList.id, newName)}
      />
      <DeleteListModal
        open={deleteListModalOpen}
        onClose={() => setDeleteListModalOpen(false)}
        onDelete={() => onDeleteTodoList(todoList.id)}
      />
    </div>
  );
}

type TodoItemProps = {
  onDelete: (todoItemId: number) => void;
  onUpdateComplete: (todoItemId: number, complete: boolean) => void;
  todoItem: TodoItem;
};

function TodoItemComponent(props: TodoItemProps) {
  const { onDelete, onUpdateComplete, todoItem } = props;

  return (
    <div className="flex flex-row gap-2 items-center">
      <Button
        className="h-7 p-0 w-7"
        icon={todoItem.complete ? <CheckIcon className="h-4 w-4" /> : undefined}
        onClick={() => onUpdateComplete(todoItem.id, !todoItem.complete)}
        variant={todoItem.complete ? 'success' : 'default'}
      />
      <p className="grow">{todoItem.text}</p>
      <Button
        icon={<MinusSmIcon className="h-4 w-4" />}
        onClick={() => onDelete(todoItem.id)}
        variant="ghost"
      />
    </div>
  );
}
