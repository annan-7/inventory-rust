// src/App.jsx
import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [notes, setNotes] = useState([]);
  const [currentNote, setCurrentNote] = useState(null);
  const [formData, setFormData] = useState({
    title: '',
    content: ''
  });
  const [isEditing, setIsEditing] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState('');

  // Fetch all notes on component mount
  useEffect(() => {
    const fetchData = async () => {
      try {
        const notes = await invoke('get_all_notes');
        setNotes(notes);
      } catch (err) {
        setError('Failed to load notes: ' + err);
      } finally {
        setIsLoading(false);
      }
    };
    
    fetchData();
  }, []);

  const fetchNotes = async () => {
    try {
      setIsLoading(true);
      const notes = await invoke('get_all_notes');
      setNotes(notes);
    } catch (err) {
      setError('Failed to load notes: ' + err);
    } finally {
      setIsLoading(false);
    }
  };

  const fetchNote = async (id) => {
    try {
      setIsLoading(true);
      const note = await invoke('get_note', { id });
      if (note) {
        setCurrentNote(note);
      }
    } catch (err) {
      setError('Failed to fetch note: ' + err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleCreateNote = async (e) => {
    e.preventDefault();
    try {
      setIsLoading(true);
      const id = await invoke('create_note', {
        payload: {
          title: formData.title,
          content: formData.content
        }
      });
      
      setFormData({ title: '', content: '' });
      await fetchNotes();
      setError('');
    } catch (err) {
      setError('Error creating note: ' + err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleUpdateNote = async () => {
    if (!currentNote) return;
    
    try {
      setIsLoading(true);
      await invoke('update_note', {
        payload: {
          id: currentNote.id,
          title: formData.title,
          content: formData.content
        }
      });
      
      setFormData({ title: '', content: '' });
      setCurrentNote(null);
      setIsEditing(false);
      await fetchNotes();
      setError('');
    } catch (err) {
      setError('Error updating note: ' + err);
    } finally {
      setIsLoading(false);
    }
  };

  const startEditing = (note) => {
    setCurrentNote(note);
    setFormData({
      title: note.title,
      content: note.content
    });
    setIsEditing(true);
  };

  const cancelEditing = () => {
    setFormData({ title: '', content: '' });
    setCurrentNote(null);
    setIsEditing(false);
    setError('');
  };

  const deleteNote = async (id) => {
    try {
      setIsLoading(true);
      await invoke('delete_note', { id });
      await fetchNotes();
      setCurrentNote(null);
      setError('');
    } catch (err) {
      setError('Error deleting note: ' + err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container">
      <div className="header">
        <h1>Notes App</h1>
      </div>
      
      {error && (
        <div className="alert alert-error">{error}</div>
      )}
      
      {isLoading && (
        <div className="loading">
          <div className="spinner"></div>
        </div>
      )}

      {/* Note Form */}
      <div className="form-container">
        <h2>{isEditing ? `Edit Note #${currentNote?.id}` : 'Create New Note'}</h2>
        
        <form onSubmit={isEditing ? handleUpdateNote : handleCreateNote}>
          <div className="form-group">
            <label htmlFor="title">Title</label>
            <input
              id="title"
              name="title"
              type="text"
              value={formData.title}
              onChange={handleInputChange}
              className="form-input"
              required
              disabled={isLoading}
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="content">Content</label>
            <textarea
              id="content"
              name="content"
              value={formData.content}
              onChange={handleInputChange}
              className="form-input"
              rows="4"
              required
              disabled={isLoading}
            ></textarea>
          </div>
          
          <div className="btn-group">
            <button
              type="submit"
              className="btn btn-primary"
              disabled={isLoading}
            >
              {isEditing ? 'Update Note' : 'Create Note'}
            </button>
            
            {isEditing && (
              <button
                type="button"
                onClick={cancelEditing}
                className="btn btn-secondary"
                disabled={isLoading}
              >
                Cancel
              </button>
            )}
          </div>
        </form>
      </div>
      
      {/* Notes List */}
      <div>
        <div className="notes-header">
          <h2>Your Notes</h2>
          <button 
            onClick={fetchNotes} 
            className="refresh-btn"
            disabled={isLoading}
          >
            Refresh
          </button>
        </div>
        
        {notes.length === 0 && !isLoading ? (
          <p>No notes yet. Create your first note!</p>
        ) : (
          <div className="notes-grid">
            {notes.map(note => (
              <div 
                key={note.id} 
                className="note-card"
                onClick={() => fetchNote(note.id)}
              >
                <div className="note-title">{note.title}</div>
                <div className="note-date">
                  {new Date(note.created_at).toLocaleString()}
                </div>
                <div className="note-content">{note.content}</div>
                
                <div className="note-actions">
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      startEditing(note);
                    }}
                    className="action-btn edit-btn"
                    disabled={isLoading}
                  >
                    Edit
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      if (confirm(`Delete note "${note.title}"?`)) {
                        deleteNote(note.id);
                      }
                    }}
                    className="action-btn delete-btn"
                    disabled={isLoading}
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
      
      {/* Note Detail Modal */}
      {currentNote && !isEditing && (
        <div className="modal-overlay">
          <div className="modal-content">
            <div className="modal-title">{currentNote.title}</div>
            <div className="modal-date">
              Created: {new Date(currentNote.created_at).toLocaleString()}
            </div>
            <div className="modal-text">{currentNote.content}</div>
            
            <div className="modal-footer">
              <button
                onClick={() => startEditing(currentNote)}
                className="btn btn-primary"
                disabled={isLoading}
              >
                Edit
              </button>
              <button
                onClick={() => {
                  if (confirm(`Delete note "${currentNote.title}"?`)) {
                    deleteNote(currentNote.id);
                  }
                }}
                className="btn btn-danger"
                disabled={isLoading}
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;